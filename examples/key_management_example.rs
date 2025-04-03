use password_manager::{
    key_management::KeyManagementPlugin,
    key_management::security::security_module::MockHSM,
    persistence::DbPersistence,
    plugin_config::PluginConfig,
    plugin_sdk::PluginSDK,
};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use tokio::signal;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 获取当前工作目录
    let current_dir = env::current_dir()?;
    let data_dir = current_dir.join("data");
    
    // 确保数据目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
        println!("创建数据目录: {:?}", data_dir);
    }
    
    // 构建数据库文件的绝对路径
    let db_path = data_dir.join("keymanagement.db");
    let db_url = format!("sqlite:{}", db_path.display());
    println!("数据库路径: {}", db_url);
    
    // 创建数据库持久化
    let db_persistence = match DbPersistence::new(&db_url).await {
        Ok(persistence) => {
            println!("数据库连接成功");
            persistence
        },
        Err(e) => {
            eprintln!("创建数据库持久化失败: {}", e);
            return Ok(());
        }
    };
    
    // 创建插件配置 - 使用正确的服务器端口
    let mut config = PluginConfig::new();
    config.set_server_host("localhost".to_string());
    config.set_server_port(19090);
    config.set_plugin_name("密钥管理插件".to_string());
    config.set_plugin_version("0.0.1".to_string());
    config.set_plugin_type("key_management".to_string());
    config.set_plugin_description("密钥管理服务".to_string()); // 确保设置描述
    
    // 简化额外配置，确保端口一致性
    config.add_config("plugin.host".to_string(), "localhost".to_string());
    config.add_config("plugin.port".to_string(), "19090".to_string()); // 确保与服务器端口一致
    config.add_config("register_retry".to_string(), "10".to_string()); // 增加重试次数
    config.add_config("register_timeout".to_string(), "60".to_string()); // 增加注册超时时间
    
    // 简化额外配置，避免重复和冲突
    config.add_config("plugin.host".to_string(), "localhost".to_string());
    config.add_config("plugin.port".to_string(), "19090".to_string());
    config.add_config("register_retry".to_string(), "5".to_string());
    
    println!("正在创建插件实例...");
    // 创建插件实例，使用数据库持久化
    let mut plugin = KeyManagementPlugin::with_security_module(Arc::new(MockHSM))
        .with_persistence(Arc::new(db_persistence));
    
    println!("正在初始化插件...");
    
    // 在这里保存配置信息，用于后续打印
    let server_host = config.get_server_host().to_string();
    let server_port = config.get_server_port();
    
    // 初始化插件前，手动设置一个插件ID用于测试
    config.set_plugin_id(format!("key_management_{}", uuid::Uuid::new_v4())); // 添加唯一ID
    
    // 初始化插件
    if !plugin.initialize(config.clone()).await {
        eprintln!("插件初始化失败");
        return Ok(());
    }
    
    // 手动设置插件信息，类似示例插件
    let mut info = plugin.get_info();
    info.set_description("密钥管理服务".to_string());
    
    // 添加支持的命令
    info.add_supported_command("create_key".to_string());
    info.add_supported_command("get_key".to_string());
    info.add_supported_command("list_keys".to_string());
    info.add_supported_command("delete_key".to_string());
    
    println!("正在启动插件并尝试注册到主服务...");
    println!("连接到服务器: {}:{}", server_host, server_port);
    
    // 使用更长的超时时间
    let start_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(60), // 增加到60秒超时
        plugin.start()
    ).await;
    
    match start_result {
        Ok(true) => {
            println!("插件已成功启动并连接到服务器");
        },
        Ok(false) => {
            eprintln!("插件启动失败，可能是服务器未运行");
            println!("将以独立模式继续运行");
        },
        Err(_) => {
            eprintln!("插件启动超时，可能是服务器无响应");
            println!("将以独立模式继续运行");
        }
    }
    
    // 使用 Arc<Mutex<>> 来共享插件实例，而不是克隆
    let plugin_arc = Arc::new(tokio::sync::Mutex::new(plugin));
    let plugin_clone = plugin_arc.clone();
    
    // 添加心跳检查 - 使用自定义心跳逻辑
    tokio::spawn(async move {
        // 先等待一段时间，让插件有机会完成注册
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        // 尝试手动注册一次
        let mut plugin = plugin_clone.lock().await;
        let plugin_id = plugin.get_info().get_id().to_string();
        
        if plugin_id.is_empty() {
            println!("检测到插件ID为空，尝试手动注册...");
            // 这里可以添加手动注册逻辑，如果您有相关API
            // 例如: plugin.register().await;
        }
        
        // 释放锁，避免死锁
        drop(plugin);
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            
            // 获取插件的锁
            let plugin = plugin_clone.lock().await;
            let plugin_id = plugin.get_info().get_id().to_string();
            let plugin_name = plugin.get_info().get_name().to_string();
            
            println!("准备发送心跳... 插件ID: {}, 插件名称: {}", 
                     if plugin_id.is_empty() { "未分配" } else { &plugin_id }, 
                     plugin_name);
            
            // 手动实现心跳逻辑
            // 由于 KeyManagementPlugin 没有 get_config 方法，我们直接使用硬编码的配置
            let server_host = "localhost"; // 使用localhost而不是0.0.0.0
            let server_port = 19090;
            // 使用 let 绑定存储 plugin_id，而不是直接使用临时值
            let plugin_id = plugin.get_info().get_id().to_string();
            
            if !plugin_id.is_empty() {
                let endpoint = format!("http://{}:{}", server_host, server_port);
                
                match Endpoint::from_shared(endpoint.clone())
                    .and_then(|endpoint| Ok(endpoint.connect_lazy()))
                {
                    Ok(channel) => {
                        use password_manager::base_plugin::plugin::plugin_service_client::PluginServiceClient;
                        use password_manager::base_plugin::plugin::HeartbeatRequest;
                        
                        let mut client = PluginServiceClient::new(channel);
                        let request = tonic::Request::new(HeartbeatRequest {
                            plugin_id: plugin_id.to_string(),
                            status_info: "RUNNING".to_string(),
                        });
                        
                        match client.heartbeat(request).await {
                            Ok(_) => println!("心跳发送成功"),
                            Err(e) => println!("心跳发送失败: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("创建gRPC客户端失败: {}", e);
                    }
                }
            } else {
                println!("插件未注册，无法发送心跳");
            }
        }
    });
    
    // 测试创建密钥 - 需要从 Arc<Mutex<>> 中获取插件实例
    let mut params = HashMap::new();
    params.insert("name".to_string(), "测试密钥".to_string());
    params.insert("description".to_string(), "这是一个测试密钥".to_string());
    params.insert("key_type".to_string(), "SYMMETRIC".to_string());
    params.insert("algorithm".to_string(), "AES-256".to_string());
    params.insert("user".to_string(), "admin".to_string());
    params.insert("tag.environment".to_string(), "test".to_string());
    
    // 获取插件的锁
    let mut plugin = plugin_arc.lock().await;
    
    let result = plugin.execute_command("create_key", &params).await;
    println!("创建密钥结果: {}", if result.is_success() { "成功" } else { "失败" });
    println!("返回数据: {}", result.get_result());
    if !result.get_error_message().is_empty() {
        println!("错误信息: {}", result.get_error_message());
    }
    
    // 等待中断信号
    println!("按Ctrl+C退出");
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("接收到中断信号，正在清理资源...");
            // 不调用 stop 方法，因为它可能会尝试与 gRPC 服务器通信
        }
        Err(e) => {
            eprintln!("无法监听中断信号: {}", e);
        }
    }
    
    Ok(())
}
