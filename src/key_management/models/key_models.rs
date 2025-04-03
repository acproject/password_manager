use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 密钥状态枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyStatus {
    Active,
    Suspended,
    Expired,
    Compromised,
    Destroyed,
    PendingDestruction,
}

impl ToString for KeyStatus {
    fn to_string(&self) -> String {
        match self {
            KeyStatus::Active => "ACTIVE".to_string(),
            KeyStatus::Suspended => "SUSPENDED".to_string(),
            KeyStatus::Expired => "EXPIRED".to_string(),
            KeyStatus::Compromised => "COMPROMISED".to_string(),
            KeyStatus::Destroyed => "DESTROYED".to_string(),
            KeyStatus::PendingDestruction => "PENDING_DESTRUCTION".to_string(),
        }
    }
}

/// 密钥类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyType {
    Symmetric,
    AsymmetricPrivate,
    AsymmetricPublic,
    HMAC,
    Password,
}

impl ToString for KeyType {
    fn to_string(&self) -> String {
        match self {
            KeyType::Symmetric => "SYMMETRIC".to_string(),
            KeyType::AsymmetricPrivate => "ASYMMETRIC_PRIVATE".to_string(),
            KeyType::AsymmetricPublic => "ASYMMETRIC_PUBLIC".to_string(),
            KeyType::HMAC => "HMAC".to_string(),
            KeyType::Password => "PASSWORD".to_string(),
        }
    }
}

/// 密钥算法枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyAlgorithm {
    AES256,
    RSA2048,
    RSA4096,
    ECDSA,
    ED25519,
}

impl ToString for KeyAlgorithm {
    fn to_string(&self) -> String {
        match self {
            KeyAlgorithm::AES256 => "AES-256".to_string(),
            KeyAlgorithm::RSA2048 => "RSA-2048".to_string(),
            KeyAlgorithm::RSA4096 => "RSA-4096".to_string(),
            KeyAlgorithm::ECDSA => "ECDSA".to_string(),
            KeyAlgorithm::ED25519 => "ED25519".to_string(),
        }
    }
}

/// 密钥元数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub key_type: KeyType,
    pub algorithm: KeyAlgorithm,
    pub status: KeyStatus,
    pub owner: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub version: u32,
    pub requires_approval: bool,
    pub tags: HashMap<String, String>,
}

impl KeyMetadata {
    pub fn new(
        name: String,
        description: String,
        key_type: KeyType,
        algorithm: KeyAlgorithm, // 这里不需要修改，但在使用时需要克隆
        owner: String,
        requires_approval: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            key_type,
            algorithm,
            status: KeyStatus::Active,
            owner,
            created_at: now,
            updated_at: now,
            expiration_date: None,
            version: 1,
            requires_approval,
            tags: HashMap::new(),
        }
    }
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub action: String,
    pub key_id: Option<String>,
    pub details: String,
    pub success: bool,
    pub error: Option<String>,
}

impl AuditLogEntry {
    pub fn new(
        action: String,
        user: String,
        key_id: Option<String>,
        details: String,
        success: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user,
            action,
            key_id,
            details,
            success,
            error: None,
        }
    }

    pub fn with_error(
        action: String,
        user: String,
        key_id: Option<String>,
        details: String,
        error: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user,
            action,
            key_id,
            details,
            success: false,
            error: Some(error),
        }
    }
}