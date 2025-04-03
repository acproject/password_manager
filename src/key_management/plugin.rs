// 添加 async_trait 导入
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::base_plugin::BasePlugin;
use crate::command_result::CommandResult;
use crate::plugin_config::PluginConfig;
use crate::plugin_sdk::PluginSDK;
use crate::persistence::PersistenceInterface;

use crate::key_management::models::key_models::{
    KeyMetadata, KeyStatus, KeyType, KeyAlgorithm, AuditLogEntry
};
use crate::key_management::security::security_module::{SecurityModuleInterface, MockHSM};

/// 密钥管理插件
pub struct KeyManagementPlugin {
    base: BasePlugin,
    keys: Arc<Mutex<HashMap<String, KeyMetadata>>>,
    audit_log: Arc<Mutex<Vec<AuditLogEntry>>>,
    security_module: Arc<dyn SecurityModuleInterface + Send + Sync>,
    pending_approvals: Arc<Mutex<HashMap<String, (String, String)>>>, // 操作ID -> (密钥ID, 操作类型)
    persistence: Option<Arc<dyn PersistenceInterface + Send + Sync>>,
}

impl KeyManagementPlugin {
    pub fn new() -> Self {
        Self {
            base: BasePlugin::new(),
            keys: Arc::new(Mutex::new(HashMap::new())),
            audit_log: Arc::new(Mutex::new(Vec::new())),
            security_module: Arc::new(MockHSM),
            pending_approvals: Arc::new(Mutex::new(HashMap::new())),
            persistence: None,
        }
    }

    pub fn with_security_module(security_module: Arc<dyn SecurityModuleInterface + Send + Sync>) -> Self {
        Self {
            base: BasePlugin::new(),
            keys: Arc::new(Mutex::new(HashMap::new())),
            audit_log: Arc::new(Mutex::new(Vec::new())),
            security_module,
            pending_approvals: Arc::new(Mutex::new(HashMap::new())),
            persistence: None,
        }
    }

    pub fn with_persistence(mut self, persistence: Arc<dyn PersistenceInterface + Send + Sync>) -> Self {
        self.persistence = Some(persistence);
        self
    }

    fn add_audit_log(&self, entry: AuditLogEntry) {
        let mut log = self.audit_log.lock().unwrap();
        log.push(entry.clone());
        
        // 如果有持久化存储，则保存审计日志
        if let Some(persistence) = &self.persistence {
            let persistence_clone = Arc::clone(persistence);
            let entry_clone = entry.clone();
            tokio::spawn(async move {
                if let Err(e) = persistence_clone.save_audit_log(&entry_clone).await {
                    eprintln!("保存审计日志失败: {}", e);
                }
            });
        }
    }

    async fn create_key(
        &self,
        name: String,
        description: String,
        key_type: KeyType,
        algorithm: KeyAlgorithm,
        owner: String,
        requires_approval: bool,
        tags: Option<HashMap<String, String>>,
        expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<KeyMetadata, String> {
        // 创建密钥元数据
        let mut metadata = KeyMetadata::new(
            name,
            description,
            key_type,
            algorithm.clone(), // 在这里克隆 algorithm
            owner.clone(),
            requires_approval,
        );
    
        // 设置标签
        if let Some(t) = tags {
            metadata.tags = t;
        }
    
        // 设置过期日期
        metadata.expiration_date = expiration_date;
    
        // 生成实际密钥
        let key_data = self.security_module.generate_key(algorithm).await?;
    
        // 存储密钥
        self.security_module.store_key(&metadata.id, &key_data).await?;
    
        // 保存元数据
        let mut keys = self.keys.lock().unwrap();
        keys.insert(metadata.id.clone(), metadata.clone());
        
        // 如果有持久化存储，则保存密钥元数据
        if let Some(persistence) = &self.persistence {
            let persistence_clone = Arc::clone(persistence);
            let metadata_clone = metadata.clone();
            tokio::spawn(async move {
                if let Err(e) = persistence_clone.save_key_metadata(&metadata_clone).await {
                    eprintln!("保存密钥元数据失败: {}", e);
                }
            });
        }
    
        // 记录审计日志
        self.add_audit_log(AuditLogEntry::new(
            "CREATE_KEY".to_string(),
            owner,
            Some(metadata.id.clone()),
            format!("Created key: {}", metadata.name),
            true,
        ));
    
        Ok(metadata)
    }

    async fn rotate_key(&self, key_id: &str, user: &str) -> Result<KeyMetadata, String> {
        // 检查密钥是否存在
        let mut keys = self.keys.lock().unwrap();
        let metadata = keys.get_mut(key_id).ok_or_else(|| "Key not found".to_string())?;

        // 检查密钥状态
        if metadata.status != KeyStatus::Active {
            return Err(format!("Key is not active, current status: {:?}", metadata.status));
        }

        // 检查是否需要审批
        if metadata.requires_approval {
            let operation_id = Uuid::new_v4().to_string();
            let mut approvals = self.pending_approvals.lock().unwrap();
            approvals.insert(operation_id.clone(), (key_id.to_string(), "ROTATE".to_string()));

            // 记录审计日志
            self.add_audit_log(AuditLogEntry::new(
                "REQUEST_KEY_ROTATION".to_string(),
                user.to_string(),
                Some(key_id.to_string()),
                format!("Requested key rotation, approval ID: {}", operation_id),
                true,
            ));

            return Err(format!("Key rotation requires approval. Approval ID: {}", operation_id));
        }

        // 生成新密钥
        let key_data = self.security_module.generate_key(metadata.algorithm.clone()).await?;

        // 存储新密钥
        self.security_module.store_key(key_id, &key_data).await?;

        // 更新元数据
        metadata.updated_at = chrono::Utc::now();
        metadata.version += 1;
        
        // 如果有持久化存储，则更新密钥元数据
        if let Some(persistence) = &self.persistence {
            let persistence_clone = Arc::clone(persistence);
            let metadata_clone = metadata.clone();
            tokio::spawn(async move {
                if let Err(e) = persistence_clone.save_key_metadata(&metadata_clone).await {
                    eprintln!("更新密钥元数据失败: {}", e);
                }
            });
        }

        // 记录审计日志
        self.add_audit_log(AuditLogEntry::new(
            "ROTATE_KEY".to_string(),
            user.to_string(),
            Some(key_id.to_string()),
            format!("Rotated key: {}", metadata.name),
            true,
        ));

        Ok(metadata.clone())
    }

    // 将 execute_command 方法改为公有
    pub async fn execute_command(&self, command: &str, params: &HashMap<String, String>) -> CommandResult {
        let user = params.get("user").cloned().unwrap_or_else(|| "system".to_string());
        
        match command {
            "create_key" => {
                let name = match params.get("name") {
                    Some(name) => name.clone(),
                    None => return CommandResult::new(false, String::new(), "Missing parameter: name".to_string()),
                };
                
                let description = params.get("description")
                    .cloned()
                    .unwrap_or_else(|| "".to_string());
                    
                let key_type_str = params.get("key_type")
                    .cloned()
                    .unwrap_or_else(|| "SYMMETRIC".to_string());
                    
                let key_type = match key_type_str.as_str() {
                    "SYMMETRIC" => KeyType::Symmetric,
                    "ASYMMETRIC_PRIVATE" => KeyType::AsymmetricPrivate,
                    "ASYMMETRIC_PUBLIC" => KeyType::AsymmetricPublic,
                    "HMAC" => KeyType::HMAC,
                    "PASSWORD" => KeyType::Password,
                    _ => return CommandResult::new(false, String::new(), format!("Invalid key type: {}", key_type_str)),
                };
                
                let algorithm_str = params.get("algorithm")
                    .cloned()
                    .unwrap_or_else(|| "AES-256".to_string());
                    
                let algorithm = match algorithm_str.as_str() {
                    "AES-256" => KeyAlgorithm::AES256,
                    "RSA-2048" => KeyAlgorithm::RSA2048,
                    "RSA-4096" => KeyAlgorithm::RSA4096,
                    "ECDSA" => KeyAlgorithm::ECDSA,
                    "ED25519" => KeyAlgorithm::ED25519,
                    _ => return CommandResult::new(false, String::new(), format!("Invalid algorithm: {}", algorithm_str)),
                };
                
                let requires_approval = params.get("requires_approval")
                    .map(|v| v.to_lowercase() == "true")
                    .unwrap_or(false);
                    
                // 收集标签
                let mut tags = HashMap::new();
                for (key, value) in params {
                    if key.starts_with("tag.") {
                        let tag_key = key.strip_prefix("tag.").unwrap();
                        tags.insert(tag_key.to_string(), value.clone());
                    }
                }
                
                match self.create_key(
                    name,
                    description,
                    key_type,
                    algorithm.clone(), // 添加 clone() 以解决所有权问题
                    user,
                    requires_approval,
                    Some(tags),
                    None,
                ).await {
                    Ok(metadata) => {
                        CommandResult::new(
                            true,
                            serde_json::to_string(&metadata).unwrap_or_default(),
                            String::new(),
                        )
                    }
                    Err(e) => CommandResult::new(false, String::new(), e),
                }
            }
            // ... 其他命令实现 ...
            _ => CommandResult::new(
                false,
                String::new(),
                format!("未知命令: {}", command),
            ),
        }
    }

    // 将 handle_message 方法改为公有
    pub async fn handle_message(&self, message: &str) -> String {
        format!("收到消息: {}", message)
    }
}

// 为 KeyManagementPlugin 实现 PluginSDK trait
#[async_trait]
impl PluginSDK for KeyManagementPlugin {
    async fn initialize(&mut self, config: PluginConfig) -> bool {
        self.base.initialize(config).await
    }

    async fn start(&mut self) -> bool {
        self.base.start().await
    }

    async fn stop(&mut self) -> bool {
        self.base.stop().await
    }

    fn get_info(&self) -> crate::plugin_info::PluginInfo {
        self.base.get_info()
    }

    async fn execute_command(&self, command: &str, params: &HashMap<String, String>) -> CommandResult {
        // 调用自己的 execute_command 方法
        self.execute_command(command, params).await
    }

    async fn handle_message(&self, message: &str) -> String {
        // 调用自己的 handle_message 方法
        self.handle_message(message).await
    }
}