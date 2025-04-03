use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::key_management_plugin::{AuditLogEntry, KeyMetadata, KeyStatus, KeyType, Algorithm, PersistenceInterface};

pub struct DbPersistence {
    pool: Pool<Sqlite>,
}

impl DbPersistence {
    pub async fn new(db_url: &str) -> Result<Self, String> {
        let pool = SqlitePool::connect(db_url)
            .await
            .map_err(|e| format!("连接数据库失败: {}", e))?;
        
        // 初始化数据库表
        Self::init_db(&pool).await?;
        
        Ok(Self { pool })
    }
    
    async fn init_db(pool: &Pool<Sqlite>) -> Result<(), String> {
        // 创建密钥元数据表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS key_metadata (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                key_type TEXT NOT NULL,
                algorithm TEXT NOT NULL,
                status TEXT NOT NULL,
                owner TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                expires_at TEXT,
                version INTEGER NOT NULL,
                requires_approval INTEGER NOT NULL
            )
            "#
        )
        .execute(pool)
        .await
        .map_err(|e| format!("创建密钥元数据表失败: {}", e))?;
        
        // 创建标签表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS key_tags (
                key_id TEXT NOT NULL,
                tag_key TEXT NOT NULL,
                tag_value TEXT NOT NULL,
                PRIMARY KEY (key_id, tag_key),
                FOREIGN KEY (key_id) REFERENCES key_metadata(id) ON DELETE CASCADE
            )
            "#
        )
        .execute(pool)
        .await
        .map_err(|e| format!("创建标签表失败: {}", e))?;
        
        // 创建审计日志表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS audit_logs (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                user TEXT NOT NULL,
                action TEXT NOT NULL,
                key_id TEXT,
                details TEXT,
                success INTEGER NOT NULL,
                error TEXT
            )
            "#
        )
        .execute(pool)
        .await
        .map_err(|e| format!("创建审计日志表失败: {}", e))?;
        
        Ok(())
    }
}

#[async_trait]
impl PersistenceInterface for DbPersistence {
    async fn save_key_metadata(&self, metadata: &KeyMetadata) -> Result<(), String> {
        // 开始事务
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| format!("开始事务失败: {}", e))?;
        
        // 保存密钥元数据
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO key_metadata 
            (id, name, description, key_type, algorithm, status, owner, created_at, updated_at, expires_at, version, requires_approval)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&metadata.id)
        .bind(&metadata.name)
        .bind(&metadata.description)
        .bind(metadata.key_type.to_string())
        .bind(metadata.algorithm.to_string())
        .bind(metadata.status.to_string())
        .bind(&metadata.owner)
        .bind(metadata.created_at.to_rfc3339())
        .bind(metadata.updated_at.to_rfc3339())
        .bind(metadata.expires_at.map(|dt| dt.to_rfc3339()))
        .bind(metadata.version)
        .bind(metadata.requires_approval as i32)
        .execute(&mut tx)
        .await
        .map_err(|e| format!("保存密钥元数据失败: {}", e))?;
        
        // 删除旧标签
        sqlx::query("DELETE FROM key_tags WHERE key_id = ?")
            .bind(&metadata.id)
            .execute(&mut tx)
            .await
            .map_err(|e| format!("删除旧标签失败: {}", e))?;
        
        // 保存新标签
        for (key, value) in &metadata.tags {
            sqlx::query(
                r#"
                INSERT INTO key_tags (key_id, tag_key, tag_value)
                VALUES (?, ?, ?)
                "#
            )
            .bind(&metadata.id)
            .bind(key)
            .bind(value)
            .execute(&mut tx)
            .await
            .map_err(|e| format!("保存标签失败: {}", e))?;
        }
        
        // 提交事务
        tx.commit()
            .await
            .map_err(|e| format!("提交事务失败: {}", e))?;
        
        Ok(())
    }
    
    async fn load_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, String> {
        // 查询密钥元数据
        let metadata = sqlx::query!(
            r#"
            SELECT * FROM key_metadata WHERE id = ?
            "#,
            key_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("查询密钥元数据失败: {}", e))?
        .ok_or_else(|| format!("密钥不存在: {}", key_id))?;
        
        // 查询标签
        let tags = sqlx::query!(
            r#"
            SELECT tag_key, tag_value FROM key_tags WHERE key_id = ?
            "#,
            key_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("查询标签失败: {}", e))?;
        
        // 构建标签映射
        let mut tag_map = HashMap::new();
        for tag in tags {
            tag_map.insert(tag.tag_key, tag.tag_value);
        }
        
        // 解析日期时间
        let created_at = DateTime::parse_from_rfc3339(&metadata.created_at)
            .map_err(|e| format!("解析创建时间失败: {}", e))?
            .with_timezone(&Utc);
        
        let updated_at = DateTime::parse_from_rfc3339(&metadata.updated_at)
            .map_err(|e| format!("解析更新时间失败: {}", e))?
            .with_timezone(&Utc);
        
        let expires_at = if let Some(expires) = metadata.expires_at {
            Some(
                DateTime::parse_from_rfc3339(&expires)
                    .map_err(|e| format!("解析过期时间失败: {}", e))?
                    .with_timezone(&Utc)
            )
        } else {
            None
        };
        
        // 构建密钥元数据
        Ok(KeyMetadata {
            id: metadata.id,
            name: metadata.name,
            description: metadata.description,
            key_type: KeyType::from_str(&metadata.key_type)
                .map_err(|_| format!("无效的密钥类型: {}", metadata.key_type))?,
            algorithm: Algorithm::from_str(&metadata.algorithm)
                .map_err(|_| format!("无效的算法: {}", metadata.algorithm))?,
            status: KeyStatus::from_str(&metadata.status)
                .map_err(|_| format!("无效的状态: {}", metadata.status))?,
            owner: metadata.owner,
            created_at,
            updated_at,
            expires_at,
            version: metadata.version,
            requires_approval: metadata.requires_approval != 0,
            tags: tag_map,
        })
    }
    
    async fn delete_key_metadata(&self, key_id: &str) -> Result<(), String> {
        // 删除密钥元数据（标签会通过外键级联删除）
        sqlx::query("DELETE FROM key_metadata WHERE id = ?")
            .bind(key_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除密钥元数据失败: {}", e))?;
        
        Ok(())
    }
    
    async fn list_key_metadata(&self, filters: Option<HashMap<String, String>>) -> Result<Vec<KeyMetadata>, String> {
        // 构建基本查询
        let mut query = "SELECT * FROM key_metadata".to_string();
        let mut params = Vec::new();
        
        // 应用过滤器
        if let Some(filters) = &filters {
            let mut where_clauses = Vec::new();
            
            for (key, value) in filters {
                match key.as_str() {
                    "status" => {
                        where_clauses.push("status = ?".to_string());
                        params.push(value.clone());
                    }
                    "type" => {
                        where_clauses.push("key_type = ?".to_string());
                        params.push(value.clone());
                    }
                    "algorithm" => {
                        where_clauses.push("algorithm = ?".to_string());
                        params.push(value.clone());
                    }
                    "owner" => {
                        where_clauses.push("owner = ?".to_string());
                        params.push(value.clone());
                    }
                    _ => {
                        // 检查是否是标签过滤器
                        if key.starts_with("tag.") {
                            let tag_key = key.strip_prefix("tag.").unwrap();
                            where_clauses.push(format!(
                                "id IN (SELECT key_id FROM key_tags WHERE tag_key = ? AND tag_value = ?)"
                            ));
                            params.push(tag_key.to_string());
                            params.push(value.clone());
                        }
                    }
                }
            }
            
            if !where_clauses.is_empty() {
                query.push_str(" WHERE ");
                query.push_str(&where_clauses.join(" AND "));
            }
        }
        
        // 执行查询
        let mut rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("查询密钥元数据失败: {}", e))?;
        
        // 构建结果
        let mut result = Vec::new();
        
        for row in rows {
            let id: String = row.get("id");
            
            // 查询标签
            let tags = sqlx::query!(
                r#"
                SELECT tag_key, tag_value FROM key_tags WHERE key_id = ?
                "#,
                id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("查询标签失败: {}", e))?;
            
            // 构建标签映射
            let mut tag_map = HashMap::new();
            for tag in tags {
                tag_map.insert(tag.tag_key, tag.tag_value);
            }
            
            // 解析日期时间
            let created_at_str: String = row.get("created_at");
            let updated_at_str: String = row.get("updated_at");
            let expires_at_str: Option<String> = row.get("expires_at");
            
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| format!("解析创建时间失败: {}", e))?
                .with_timezone(&Utc);
            
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| format!("解析更新时间失败: {}", e))?
                .with_timezone(&Utc);
            
            let expires_at = if let Some(expires) = expires_at_str {
                Some(
                    DateTime::parse_from_rfc3339(&expires)
                        .map_err(|e| format!("解析过期时间失败: {}", e))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            // 构建密钥元数据
            let metadata = KeyMetadata {
                id,
                name: row.get("name"),
                description: row.get("description"),
                key_type: KeyType::from_str(&row.get::<String, _>("key_type"))
                    .map_err(|_| format!("无效的密钥类型"))?,
                algorithm: Algorithm::from_str(&row.get::<String, _>("algorithm"))
                    .map_err(|_| format!("无效的算法"))?,
                status: KeyStatus::from_str(&row.get::<String, _>("status"))
                    .map_err(|_| format!("无效的状态"))?,
                owner: row.get("owner"),
                created_at,
                updated_at,
                expires_at,
                version: row.get("version"),
                requires_approval: row.get::<i32, _>("requires_approval") != 0,
                tags: tag_map,
            };
            
            result.push(metadata);
        }
        
        Ok(result)
    }
    
    async fn save_audit_log(&self, log: &AuditLogEntry) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs 
            (id, timestamp, user, action, key_id, details, success, error)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&log.id)
        .bind(log.timestamp.to_rfc3339())
        .bind(&log.user)
        .bind(&log.action)
        .bind(&log.key_id)
        .bind(&log.details)
        .bind(log.success as i32)
        .bind(&log.error)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存审计日志失败: {}", e))?;
        
        Ok(())
    }
    
    async fn load_audit_logs(&self, filters: Option<HashMap<String, String>>, limit: Option<usize>) -> Result<Vec<AuditLogEntry>, String> {
        // 构建基本查询
        let mut query = "SELECT * FROM audit_logs".to_string();
        let mut params = Vec::new();
        
        // 应用过滤器
        if let Some(filters) = &filters {
            let mut where_clauses = Vec::new();
            
            for (key, value) in filters {
                match key.as_str() {
                    "action" => {
                        where_clauses.push("action = ?".to_string());
                        params.push(value.clone());
                    }
                    "user" => {
                        where_clauses.push("user = ?".to_string());
                        params.push(value.clone());
                    }
                    "key_id" => {
                        where_clauses.push("key_id = ?".to_string());
                        params.push(value.clone());
                    }
                    "success" => {
                        let success_value = value.parse::<bool>().unwrap_or(false);
                        where_clauses.push("success = ?".to_string());
                        params.push((success_value as i32).to_string());
                    }
                    _ => {}
                }
            }
            
            if !where_clauses.is_empty() {
                query.push_str(" WHERE ");
                query.push_str(&where_clauses.join(" AND "));
            }
        }
        
        // 添加排序和限制
        query.push_str(" ORDER BY timestamp DESC");
        
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        // 执行查询
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("查询审计日志失败: {}", e))?;
        
        // 构建结果
        let mut result = Vec::new();
        
        for row in rows {
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| format!("解析时间戳失败: {}", e))?
                .with_timezone(&Utc);
            
            let log = AuditLogEntry {
                id: row.get("id"),
                timestamp,
                user: row.get("user"),
                action: row.get("action"),
                key_id: row.get("key_id"),
                details: row.get("details"),
                success: row.get::<i32, _>("success") != 0,
                error: row.get("error"),
            };
            
            result.push(log);
        }
        
        Ok(result)
    }
}