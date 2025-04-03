use async_trait::async_trait;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// 修改导入路径，使用新的模块结构
use crate::key_management::models::key_models::{AuditLogEntry, KeyMetadata};
use crate::persistence::PersistenceInterface;

pub struct FilePersistence {
    metadata_dir: String,
    audit_log_file: String,
}

impl FilePersistence {
    pub fn new(base_dir: &str) -> Self {
        let metadata_dir = format!("{}/metadata", base_dir);
        let audit_log_file = format!("{}/audit.log", base_dir);
        
        // 确保目录存在
        std::fs::create_dir_all(&metadata_dir).unwrap_or_else(|e| {
            eprintln!("创建元数据目录失败: {}", e);
        });
        
        Self {
            metadata_dir,
            audit_log_file,
        }
    }
}

#[async_trait]
impl PersistenceInterface for FilePersistence {
    async fn save_key_metadata(&self, metadata: &KeyMetadata) -> Result<(), String> {
        let file_path = format!("{}/{}.json", self.metadata_dir, metadata.id);
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| format!("序列化元数据失败: {}", e))?;
        
        fs::write(&file_path, json)
            .map_err(|e| format!("写入元数据文件失败: {}", e))?;
        
        Ok(())
    }
    
    async fn load_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, String> {
        let file_path = format!("{}/{}.json", self.metadata_dir, key_id);
        let json = fs::read_to_string(&file_path)
            .map_err(|e| format!("读取元数据文件失败: {}", e))?;
        
        serde_json::from_str(&json)
            .map_err(|e| format!("解析元数据失败: {}", e))
    }
    
    async fn delete_key_metadata(&self, key_id: &str) -> Result<(), String> {
        let file_path = format!("{}/{}.json", self.metadata_dir, key_id);
        
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path)
                .map_err(|e| format!("删除元数据文件失败: {}", e))?;
        }
        
        Ok(())
    }
    
    async fn list_key_metadata(&self, filters: Option<HashMap<String, String>>) -> Result<Vec<KeyMetadata>, String> {
        let mut result = Vec::new();
        
        let entries = fs::read_dir(&self.metadata_dir)
            .map_err(|e| format!("读取元数据目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let json = fs::read_to_string(&path)
                    .map_err(|e| format!("读取元数据文件失败: {}", e))?;
                
                let metadata: KeyMetadata = serde_json::from_str(&json)
                    .map_err(|e| format!("解析元数据失败: {}", e))?;
                
                // 应用过滤器
                if let Some(filters) = &filters {
                    let mut match_all = true;
                    
                    for (key, value) in filters {
                        match key.as_str() {
                            "status" => {
                                if metadata.status.to_string() != *value {
                                    match_all = false;
                                    break;
                                }
                            }
                            "type" => {
                                if metadata.key_type.to_string() != *value {
                                    match_all = false;
                                    break;
                                }
                            }
                            "algorithm" => {
                                if metadata.algorithm.to_string() != *value {
                                    match_all = false;
                                    break;
                                }
                            }
                            "owner" => {
                                if metadata.owner != *value {
                                    match_all = false;
                                    break;
                                }
                            }
                            _ => {
                                // 检查是否是标签过滤器
                                if key.starts_with("tag.") {
                                    let tag_key = key.strip_prefix("tag.").unwrap();
                                    match metadata.tags.get(tag_key) {
                                        Some(tag_value) if tag_value == value => {}
                                        _ => {
                                            match_all = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if !match_all {
                        continue;
                    }
                }
                
                result.push(metadata);
            }
        }
        
        Ok(result)
    }
    
    async fn save_audit_log(&self, log: &AuditLogEntry) -> Result<(), String> {
        let json = serde_json::to_string(log)
            .map_err(|e| format!("序列化审计日志失败: {}", e))?;
        
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_log_file)
            .map_err(|e| format!("打开审计日志文件失败: {}", e))?;
        
        writeln!(file, "{}", json)
            .map_err(|e| format!("写入审计日志失败: {}", e))?;
        
        Ok(())
    }
    
    async fn load_audit_logs(&self, filters: Option<HashMap<String, String>>, limit: Option<usize>) -> Result<Vec<AuditLogEntry>, String> {
        let mut result = Vec::new();
        
        if !Path::new(&self.audit_log_file).exists() {
            return Ok(result);
        }
        
        let file = File::open(&self.audit_log_file)
            .map_err(|e| format!("打开审计日志文件失败: {}", e))?;
        
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("读取审计日志行失败: {}", e))?;
            
            let log: AuditLogEntry = serde_json::from_str(&line)
                .map_err(|e| format!("解析审计日志失败: {}", e))?;
            
            // 应用过滤器
            if let Some(filters) = &filters {
                let mut match_all = true;
                
                for (key, value) in filters {
                    match key.as_str() {
                        "action" => {
                            if log.action != *value {
                                match_all = false;
                                break;
                            }
                        }
                        "user" => {
                            if log.user != *value {
                                match_all = false;
                                break;
                            }
                        }
                        "key_id" => {
                            if log.key_id.as_deref() != Some(value) {
                                match_all = false;
                                break;
                            }
                        }
                        "success" => {
                            let success_value = value.parse::<bool>().unwrap_or(false);
                            if log.success != success_value {
                                match_all = false;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                
                if !match_all {
                    continue;
                }
            }
            
            result.push(log);
        }
        
        // 应用限制
        if let Some(limit) = limit {
            if result.len() > limit {
                result.truncate(limit);
            }
        }
        
        Ok(result)
    }
}