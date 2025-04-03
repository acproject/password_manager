pub mod file_persistence;
pub mod db_persistence;

use async_trait::async_trait;
use std::collections::HashMap;
// 修改导入路径，使用新的模块结构
use crate::key_management::models::key_models::{AuditLogEntry, KeyMetadata};

#[async_trait]
pub trait PersistenceInterface: Send + Sync {
    async fn save_key_metadata(&self, metadata: &KeyMetadata) -> Result<(), String>;
    async fn load_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, String>;
    async fn delete_key_metadata(&self, key_id: &str) -> Result<(), String>;
    async fn list_key_metadata(&self, filters: Option<HashMap<String, String>>) -> Result<Vec<KeyMetadata>, String>;
    async fn save_audit_log(&self, log: &AuditLogEntry) -> Result<(), String>;
    async fn load_audit_logs(&self, filters: Option<HashMap<String, String>>, limit: Option<usize>) -> Result<Vec<AuditLogEntry>, String>;
}

pub use file_persistence::FilePersistence;
pub use db_persistence::DbPersistence;