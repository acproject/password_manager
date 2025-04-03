use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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

    async fn create_client(&self) -> Result<PluginServiceClient<Channel>, Box<dyn std::error::Error>> {
        let config = self.config.as_ref().ok_or("Plugin not initialized")?;
        let endpoint = format!("http://{}:{}", config.get_server_host(), config.get_server_port());
        let channel = Endpoint::from_shared(endpoint)?.connect().await?;
        Ok(PluginServiceClient::new(channel))
    }

    async fn heartbeat_loop(
        plugin_id: String,
        status: String,
        running: Arc<Mutex<bool>>,
        mut shutdown_rx: mpsc::Receiver<()>,
        server_host: String,
        server_port: i32,
    ) {
        let endpoint = format!("http://{}:{}", server_host, server_port);
        
        loop {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    let is_running = {
                        let guard = running.lock().unwrap();
                        *guard
                    };

                    if !is_running {
                        break;
                    }

                    match Endpoint::from_shared(endpoint.clone())
                        .and_then(|endpoint| Ok(endpoint.connect_lazy()))
                    {
                        Ok(channel) => {
                            let mut client = PluginServiceClient::new(channel);
                            let request = tonic::Request::new(HeartbeatRequest {
                                plugin_id: plugin_id.clone(),
                                status_info: status.clone(),
                            });

                            match client.heartbeat(request).await {
                                Ok(_) => {
                                    // 心跳成功
                                }
                                Err(e) => {
                                    eprintln!("心跳发送失败: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("创建gRPC客户端失败: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
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
            return false;
        }
        
        // 获取必要的配置信息
        let server_host;
        let server_port;
        let _plugin_id; // 添加下划线前缀表示有意不使用
        let plugin_name;
        let plugin_version;
        let plugin_description;
        let plugin_type;
        
        // 使用作用域来限制不可变借用
        {
            let config = self.config.as_ref().unwrap();
            server_host = config.get_server_host().to_string();
            server_port = config.get_server_port();
            _plugin_id = self.info.get_id().to_string(); // 添加下划线前缀
            plugin_name = self.info.get_name().to_string();
            plugin_version = self.info.get_version().to_string();
            plugin_description = self.info.get_description().to_string();
            plugin_type = self.info.get_type().to_string();
        }
        
        // 创建连接字符串
        let conn_str = format!("http://{}:{}", server_host, server_port);
        
        // 连接到服务器
        match Endpoint::from_shared(conn_str)
            .and_then(|endpoint| Ok(endpoint.connect_lazy()))
        {
            Ok(channel) => {
                let mut client = PluginServiceClient::new(channel);
                
                // 创建注册请求，根据实际的 PluginRegistration 结构调整字段
                let request = Request::new(PluginRegistration {
                    name: plugin_name,
                    version: plugin_version,
                    r#type: plugin_type,
                    description: plugin_description,
                    host: "localhost".to_string(), // 默认值
                    port: 50052, // 默认值
                });
                
                // 发送注册请求
                match client.register_plugin(request).await {
                    Ok(response) => {
                        let response = response.into_inner();
                        if response.success {
                            println!("插件注册成功: {}", response.message);
                            
                            // 更新配置中的注册状态
                            if let Some(config) = &mut self.config {
                                // 这里不再有借用冲突，因为前面的不可变借用已经结束
                                config.set_plugin_id(response.plugin_id.clone());
                            }
                            
                            return true;
                        } else {
                            eprintln!("插件注册失败: {}", response.message);
                        }
                    }
                    Err(e) => {
                        eprintln!("注册请求失败: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("连接到服务器失败: {}", e);
            }
        }
        
        false
    }
    
    // 添加心跳方法
    // 在 send_heartbeat 方法中
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
        let max_retries = match &self.config {
            Some(config) => config.get_config("register_retry")
                .map(|s| s.parse::<u32>().unwrap_or(3))
                .unwrap_or(3),
            None => 3, // 默认值
        };
            
        for i in 0..max_retries {
            println!("尝试注册插件 (尝试 {}/{})", i+1, max_retries);
            
            // 调用已有的注册方法
            if self.register_with_server().await {
                println!("注册成功，插件ID: {}", self.info.get_id());
                return Ok(());
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        
        Err("注册失败，已达到最大重试次数".to_string())
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

        // 创建gRPC客户端
        let mut client = match self.create_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("创建gRPC客户端失败: {}", e);
                return false;
            }
        };

        // 注册插件
        let plugin_host = match config_clone.get_config("plugin.host") {
            Some(host) => host.clone(),
            None => "localhost".to_string(),
        };

        let plugin_port = match config_clone.get_config("plugin.port") {
            Some(port) => port.parse::<i32>().unwrap_or(50052),
            None => 50052,
        };

        let request = tonic::Request::new(PluginRegistration {
            name: self.info.get_name().to_string(),
            version: self.info.get_version().to_string(),
            r#type: self.info.get_type().to_string(),
            description: self.info.get_description().to_string(),
            host: plugin_host,
            port: plugin_port,
        });

        let response = match client.register_plugin(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("插件注册失败: {}", e);
                return false;
            }
        };

        if !response.success {
            eprintln!("插件注册失败: {}", response.message);
            return false;
        }

        // 设置插件ID
        self.info.set_id(response.plugin_id.clone());
        if let Some(config) = &mut self.config {
            config.set_plugin_id(response.plugin_id.clone());
        }

        // 启动心跳线程
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let plugin_id = self.info.get_id().to_string();
        let status = self.info.get_status().to_string();
        let running = Arc::clone(&self.running);
        let server_host = config_clone.get_server_host().to_string();
        let server_port = config_clone.get_server_port();

        let handle = tokio::spawn(async move {
            Self::heartbeat_loop(
                plugin_id,
                status,
                running,
                shutdown_rx,
                server_host,
                server_port,
            )
            .await;
        });

        self.heartbeat_handle = Some(handle);

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

// 删除这些多余的导入和函数定义
// 删除下面的代码
// 删除文件末尾的重复导入注释