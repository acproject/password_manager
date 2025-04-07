use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// 使用tokio的Duration而不是std的Duration
use tokio::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tonic::transport::{Channel, Endpoint};
use tonic::Request; // 添加这一行导入

use crate::command_result::CommandResult;
use crate::plugin_config::PluginConfig;
use crate::plugin_info::PluginInfo;
use crate::plugin_sdk::PluginSDK;

// 导入生成的protobuf代码
pub mod plugin {
    tonic::include_proto!("plugin");
}

use plugin::plugin_service_client::PluginServiceClient;
use plugin::{
    HeartbeatRequest, PluginRegistration, StopRequest,
};

/// 基础插件实现
pub struct BasePlugin {
    config: Option<PluginConfig>,
    info: PluginInfo,
    running: Arc<Mutex<bool>>,
    heartbeat_handle: Option<JoinHandle<()>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

// 在 BasePlugin 结构体中添加心跳和重试注册的方法

impl BasePlugin {
    pub fn new() -> Self {
        Self {
            config: None,
            info: PluginInfo::new(),
            running: Arc::new(Mutex::new(false)),
            heartbeat_handle: None,
            shutdown_tx: None,
        }
    }

    async fn create_client(&self) -> Result<PluginServiceClient<Channel>, Box<dyn std::error::Error + Send + Sync>> {
        let config = self.config.as_ref().ok_or("Plugin not initialized")?;
        let endpoint = format!("http://{}:{}", config.get_server_host(), config.get_server_port());
        println!("尝试连接到gRPC服务器: {}", endpoint);
        
        // 修改连接方式，使用connect()而不是connect_lazy()
        println!("使用connect()方法建立连接");
        let channel = Endpoint::from_shared(endpoint)?
            .timeout(std::time::Duration::from_secs(30)) // 增加超时时间
            .connect_timeout(std::time::Duration::from_secs(15)) // 增加连接超时
            .tcp_keepalive(Some(std::time::Duration::from_secs(60))) // 增加TCP保活时间
            .connect()
            .await?;
            
        println!("gRPC连接建立成功");
        Ok(PluginServiceClient::new(channel))
    }

    // 1. 修复 heartbeat_loop 函数，添加缺失的变量定义
    async fn heartbeat_loop(
        plugin_id: String,
        status: String,
        running: Arc<Mutex<bool>>,
        mut shutdown_rx: mpsc::Receiver<()>,
        server_host: String,
        server_port: i32,
        retry_registration: bool, // 添加重试注册标志
        plugin_name: String,      // 添加插件信息
        plugin_version: String,
        plugin_type: String,
        plugin_description: String,
        host_address: String,
        plugin_grpc_port: i32,
    ) {
        let endpoint = format!("http://{}:{}", server_host, server_port);
        println!("心跳线程启动，连接到: {}", endpoint);
        
        // 添加注册重试标志和计数器
        let mut _registration_retried = false; // 添加下划线前缀表示有意不使用
        let mut retry_count = 0;
        let max_retries = 3; // 最大重试次数
        
        loop {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    let is_running = {
                        let guard = running.lock().unwrap();
                        *guard
                    };
        
                    if !is_running {
                        println!("插件已停止，心跳线程退出");
                        break;
                    }
        
                    // 使用connect()而不是connect_lazy()
                    match Endpoint::from_shared(endpoint.clone()) {
                        Ok(endpoint) => {
                            match endpoint
                                .timeout(std::time::Duration::from_secs(10))
                                .connect_timeout(std::time::Duration::from_secs(5))
                                .connect()
                                .await
                            {
                                Ok(channel) => {
                                    let mut client = PluginServiceClient::new(channel);
                                    
                                    // 发送心跳
                                    let request = tonic::Request::new(HeartbeatRequest {
                                        plugin_id: plugin_id.clone(),
                                        status_info: status.clone(),
                                    });
        
                                    match client.heartbeat(request).await {
                                        Ok(_) => {
                                            println!("心跳发送成功");
                                            
                                            // 如果需要重试注册且尚未达到最大重试次数
                                            if retry_registration && retry_count < max_retries && plugin_id.contains("-") {
                                                println!("心跳成功，尝试重新注册插件 (尝试 {}/{})", retry_count + 1, max_retries);
                                                retry_count += 1;
                                                
                                                // 创建完整的注册请求
                                                let request = tonic::Request::new(PluginRegistration {
                                                    name: plugin_name.clone(),
                                                    version: plugin_version.clone(),
                                                    r#type: plugin_type.clone(),
                                                    description: plugin_description.clone(),
                                                    host: host_address.clone(),
                                                    port: plugin_grpc_port,
                                                });
                                                
                                                println!("重新发送注册请求: name={}, version={}, type={}, description={}, host={}, port={}",
                                                         plugin_name, plugin_version, plugin_type, plugin_description, host_address, plugin_grpc_port);
                                                
                                                // 直接发送注册请求，不使用timeout包装
                                                match client.register_plugin(request).await {
                                                    Ok(response) => {
                                                        let response = response.into_inner();
                                                        if response.success {
                                                            println!("插件重新注册成功: {}", response.message);
                                                            println!("新插件ID: {}", response.plugin_id);
                                                            _registration_retried = true; // 使用修改后的变量名
                                                            retry_count = max_retries; // 不再重试
                                                        } else {
                                                            eprintln!("插件重新注册失败: {}", response.message);
                                                        }
                                                    },
                                                    Err(e) => {
                                                        eprintln!("插件重新注册失败: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("心跳发送失败: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("心跳连接失败: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("创建心跳Endpoint失败: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    println!("收到关闭信号，心跳线程退出");
                    break;
                }
            }
        }
    }
    
    // 将 register_with_server 方法移到 impl 块内部
    async fn register_with_server(&mut self) -> bool {
        // 首先检查配置是否存在
        if self.config.is_none() {
            eprintln!("插件配置未初始化");
            return true; // 返回true以允许独立模式运行
        }
        
        // 获取必要的配置信息
        let server_host;
        let server_port;
        let plugin_name;
        let plugin_version;
        let plugin_description;
        let plugin_type;
        let host_address;
        let plugin_grpc_port;
        
        // 使用作用域来限制不可变借用
        {
            let config = self.config.as_ref().unwrap();
            server_host = config.get_server_host().to_string();
            server_port = config.get_server_port();
            plugin_name = self.info.get_name().to_string();
            plugin_version = self.info.get_version().to_string();
            plugin_description = self.info.get_description().to_string();
            plugin_type = self.info.get_type().to_string();
            
            // 获取本地主机地址
            host_address = config.get_config("host_address")
                .map(|s| s.to_string())
                .unwrap_or_else(|| "localhost".to_string());
                
            // 获取插件的gRPC端口
            plugin_grpc_port = config.get_config("plugin_grpc_port")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(19091); // 默认使用19091作为插件自身的gRPC端口
        }
        
        // 创建连接字符串
        let conn_str = format!("http://{}:{}", server_host, server_port);
        println!("尝试注册到服务器: {}", conn_str);
        
        // 使用create_client方法创建客户端
        match self.create_client().await {
            Ok(mut client) => {
                println!("gRPC客户端创建成功，准备发送注册请求");
                
                // 创建完整的注册请求，确保与SysPlugin.java中的字段一致
                let request = Request::new(PluginRegistration {
                    name: plugin_name.clone(),
                    version: plugin_version.clone(),
                    r#type: plugin_type.clone(),
                    description: plugin_description.clone(), // 使用完整描述
                    host: host_address.clone(), // 设置主机地址
                    port: plugin_grpc_port, // 设置插件自身的gRPC端口
                });
                
                println!("发送注册请求: name={}, version={}, type={}, description={}, host={}, port={}",
                         plugin_name, plugin_version, plugin_type, plugin_description, host_address, plugin_grpc_port);
                
                // 直接发送注册请求，不使用timeout包装
                match client.register_plugin(request).await {
                    Ok(response) => {
                        let response = response.into_inner();
                        if response.success {
                            println!("插件注册成功: {}", response.message);
                            
                            // 更新配置中的注册状态
                            if let Some(config) = &mut self.config {
                                config.set_plugin_id(response.plugin_id.clone());
                            }
                            self.info.set_id(response.plugin_id.clone());
                            
                            return true;
                        } else {
                            eprintln!("插件注册失败: {}", response.message);
                            // 生成本地ID
                            use uuid::Uuid;
                            let local_id = Uuid::new_v4().to_string();
                            self.info.set_id(local_id.clone());
                            if let Some(config) = &mut self.config {
                                config.set_plugin_id(local_id.clone()); // 添加 clone() 以避免移动
                            }
                            println!("生成本地插件ID: {}", self.info.get_id());
                            return true;
                        }
                    },
                    Err(e) => {
                        eprintln!("插件注册失败: {}", e);
                        eprintln!("错误详情: {:?}", e);
                        // 生成本地ID
                        use uuid::Uuid;
                        let local_id = Uuid::new_v4().to_string();
                        self.info.set_id(local_id.clone());
                        if let Some(config) = &mut self.config {
                            config.set_plugin_id(local_id.clone()); // 添加 clone()
                        }
                        println!("生成本地插件ID: {}", self.info.get_id());
                        return true;
                    }
                }
            }
            Err(e) => {
                eprintln!("创建gRPC客户端失败: {}", e);
                // 生成本地ID
                use uuid::Uuid;
                let local_id = Uuid::new_v4().to_string();
                self.info.set_id(local_id.clone());
                if let Some(config) = &mut self.config {
                    config.set_plugin_id(local_id.clone()); // 添加 clone() 以避免移动
                }
                println!("生成本地插件ID: {}", local_id);
                return true;
            }
        }
    }
    
    // 添加心跳方法
    pub async fn send_heartbeat(&self) -> Result<bool, String> {
        if self.info.get_id().is_empty() {
            return Err("插件未注册，无法发送心跳".to_string());
        }
        
        // 创建gRPC客户端
        match self.create_client().await {
            Ok(mut client) => {
                // 获取当前运行状态
                let status = {
                    let guard = self.running.lock().unwrap();
                    if *guard { "RUNNING" } else { "STOPPED" }
                };
                
                let request = tonic::Request::new(HeartbeatRequest {
                    plugin_id: self.info.get_id().to_string(),
                    status_info: status.to_string(), // 使用实际运行状态
                });
        
                match client.heartbeat(request).await {
                    Ok(_) => {
                        println!("心跳发送成功，状态: {}", status);
                        Ok(true)
                    },
                    Err(e) => Err(format!("心跳发送失败: {}", e))
                }
            },
            Err(e) => Err(format!("创建gRPC客户端失败: {}", e))
        }
    }
    
    // 添加重试注册方法
    pub async fn retry_register(&mut self) -> Result<(), String> {
        // 增加默认重试次数到5次
        let max_retries = match &self.config {
            Some(config) => config.get_config("register_retry")
                .map(|s| s.parse::<u32>().unwrap_or(5))
                .unwrap_or(5),
            None => 5, // 默认值从3增加到5
        };
            
        // 增加重试间隔，从2秒增加到3秒
        let retry_interval = match &self.config {
            Some(config) => config.get_config("register_retry_interval")
                .map(|s| s.parse::<u64>().unwrap_or(3))
                .unwrap_or(3),
            None => 3, // 默认值
        };
        
        println!("开始注册插件，最大尝试次数: {}，重试间隔: {}秒", max_retries, retry_interval);
            
        for i in 0..max_retries {
            println!("尝试注册插件 (尝试 {}/{})", i+1, max_retries);
            
            // 调用已有的注册方法
            if self.register_with_server().await {
                println!("注册成功，插件ID: {}", self.info.get_id());
                return Ok(());
            }
            
            // 最后一次尝试后不需要等待
            if i < max_retries - 1 {
                println!("注册失败，{}秒后重试...", retry_interval);
                tokio::time::sleep(tokio::time::Duration::from_secs(retry_interval)).await;
            }
        }
        
        // 即使注册失败，我们仍然可以以本地模式运行
        println!("注册失败，已达到最大重试次数 {}，将以本地模式运行", max_retries);
        
        // 确保我们有一个有效的本地ID
        if self.info.get_id().is_empty() {
            use uuid::Uuid;
            let local_id = Uuid::new_v4().to_string();
            self.info.set_id(local_id.clone());
            if let Some(config) = &mut self.config {
                config.set_plugin_id(local_id.clone());
            }
            println!("生成本地插件ID: {}", local_id);
        }
        
        // 返回Ok而不是Err，因为我们可以以本地模式运行
        Ok(())
    }
}

#[async_trait]
impl PluginSDK for BasePlugin {
    async fn initialize(&mut self, config: PluginConfig) -> bool {
        self.config = Some(config.clone());
        
        // 设置插件基本信息
        self.info.set_id(config.get_plugin_id().to_string());
        self.info.set_name(config.get_plugin_name().to_string());
        self.info.set_version(config.get_plugin_version().to_string());
        self.info.set_type(config.get_plugin_type().to_string());
        
        true
    }

    async fn start(&mut self) -> bool {
        let is_running = {
            let mut guard = self.running.lock().unwrap();
            if *guard {
                return true;
            }
            *guard = true;
            true
        };
    
        if !is_running {
            return false;
        }
    
        // 创建一个配置的副本，避免后面的借用冲突
        let config_clone = match &self.config {
            Some(config) => config.clone(),
            None => return false,
        };
    
        // 尝试注册插件
        println!("尝试注册插件...");
        let registration_success = self.register_with_server().await;
        
        // 如果注册失败，我们将在心跳中重试
        let retry_registration = !registration_success || self.info.get_id().contains("-");
    
        // 如果插件ID为空，生成一个本地ID
        if self.info.get_id().is_empty() {
            use uuid::Uuid;
            let local_id = Uuid::new_v4().to_string();
            self.info.set_id(local_id.clone());
            if let Some(config) = &mut self.config {
                config.set_plugin_id(local_id.clone());
            }
            println!("生成本地插件ID: {}", local_id);
        }
    
        // 启动心跳线程
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);
    
        let plugin_id = self.info.get_id().to_string();
        let status = self.info.get_status().to_string();
        let running = Arc::clone(&self.running);
        let server_host = config_clone.get_server_host().to_string();
        let server_port = config_clone.get_server_port();
    
        // 添加插件信息用于重新注册
        let plugin_name = self.info.get_name().to_string();
        let plugin_version = self.info.get_version().to_string();
        let plugin_type = self.info.get_type().to_string();
        let plugin_description = self.info.get_description().to_string();
    
        // 获取主机地址和端口
        let host_address = config_clone.get_config("host_address")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "localhost".to_string());
            
        let plugin_grpc_port = config_clone.get_config("plugin_grpc_port")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(19091);
    
        let handle = tokio::spawn(async move {
            Self::heartbeat_loop(
                plugin_id,
                status,
                running,
                shutdown_rx,
                server_host,
                server_port,
                retry_registration,
                plugin_name,
                plugin_version,
                plugin_type,
                plugin_description,
                host_address,
                plugin_grpc_port,
            )
            .await;
        });
    
        self.heartbeat_handle = Some(handle);
        println!("插件已启动，ID: {}", self.info.get_id());
    
        true
    }

    async fn stop(&mut self) -> bool {
        let was_running = {
            let mut guard = self.running.lock().unwrap();
            let was_running = *guard;
            *guard = false;
            was_running
        };

        if !was_running {
            return true;
        }

        // 停止心跳线程
        if let Some(tx) = &self.shutdown_tx {
            let _ = tx.send(()).await;
        }

        if let Some(handle) = self.heartbeat_handle.take() {
            let _ = handle.await;
        }

        // 通知服务器停止插件
        if let Some(client_result) = self.create_client().await.ok() {
            let mut client = client_result;
            let request = tonic::Request::new(StopRequest {
                plugin_id: self.info.get_id().to_string(),
            });

            let _ = client.stop_plugin(request).await;
        }

        true
    }

    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn execute_command(&self, command: &str, _params: &HashMap<String, String>) -> CommandResult {
        // 添加下划线前缀表示有意不使用该变量
        CommandResult::new(
            false,
            String::new(),
            format!("不支持的命令: {}", command),
        )
    }

    async fn handle_message(&self, message: &str) -> String {
        // 子类应该重写此方法
        format!("未处理的消息: {}", message)
    }
}

