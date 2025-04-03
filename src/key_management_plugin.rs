// 重导出新模块结构中的内容
pub use crate::key_management::models::key_models::{
    KeyMetadata, KeyStatus, KeyType, KeyAlgorithm, AuditLogEntry
};
pub use crate::key_management::security::security_module::{SecurityModuleInterface, MockHSM};
pub use crate::key_management::plugin::KeyManagementPlugin;