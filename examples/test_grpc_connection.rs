use tonic::transport::{Channel, Endpoint};
use std::time::Duration;

// 导入生成的protobuf代码
mod plugin {
    tonic::include_proto!("plugin");
}

use plugin::plugin_service_client::PluginServiceClient;
use plugin::{PluginRegistration, HeartbeatRequest};

// 添加测试心跳的函数
async fn test_heartbeat(client: &mut PluginServiceClient<Channel>, plugin_id: &str) {
    println!("测试心跳功能...");
    
    // 创建心跳请求
    let request = tonic::Request::new(HeartbeatRequest {
        plugin_id: plugin_id.to_string(),
        status_info: "TESTING".to_string(),
    });
    
    // 发送心跳请求
    match client.heartbeat(request).await {
        Ok(response) => {
            let response = response.into_inner();
            println!("心跳响应: received={}, server_time={}", 
                     response.received, 
                     response.server_time);
        },
        Err(e) => {
            println!("心跳请求失败: {}", e);
            println!("错误详情: {:?}", e);
        }
    }
}

// 添加一个使用TCP直接连接测试的函数
async fn test_tcp_connection(host: &str, port: u16) -> bool {
    println!("使用TCP直接连接测试 {}:{}...", host, port);
    
    match tokio::net::TcpStream::connect(format!("{}:{}", host, port)).await {
        Ok(_) => {
            println!("TCP连接成功，服务器端口已开放");
            true
        },
        Err(e) => {
            println!("TCP连接失败: {}", e);
            false
        }
    }
}

// 添加测试注册的函数
async fn test_register(client: &mut PluginServiceClient<Channel>, name: &str, description: &str) -> String {
    println!("测试注册功能: name={}, description={}", name, description);
    
    // 创建注册请求 - 修改为与Java版本完全一致
    let mut request = tonic::Request::new(PluginRegistration {
        name: name.to_string(),
        version: "0.0.1".to_string(),
        r#type: "password".to_string(),
        description: description.to_string(),
        host: "localhost".to_string(),
        port: 19091,
    });
    
    // 添加元数据，模拟Java客户端的行为
    let metadata = request.metadata_mut();
    metadata.insert("content-type", "application/grpc".parse().unwrap());
    metadata.insert("user-agent", "grpc-java/1.0".parse().unwrap());
    
    // 发送注册请求，不使用超时，与Java行为保持一致
    match client.register_plugin(request).await {
        Ok(response) => {
            let response = response.into_inner();
            println!("注册响应: 成功={}, 消息={}, 插件ID={}", 
                    response.success, 
                    response.message, 
                    response.plugin_id);
            
            // 返回插件ID
            response.plugin_id
        },
        Err(e) => {
            println!("注册请求失败: {}", e);
            println!("错误详情: {:?}", e);
            "".to_string()
        }
    }
}

// 使用已建立的连接进行测试
async fn test_with_channel(channel: Channel) {
    println!("gRPC连接建立成功，开始测试");
    
    let mut client = PluginServiceClient::new(channel);
    
    // 测试标准注册
    let plugin_id = test_register(&mut client, "测试插件", "这是一个测试插件").await;
    
    // 如果标准注册失败，尝试简化版注册
    if plugin_id.is_empty() {
        println!("\n尝试使用简化版注册请求...");
        let simple_plugin_id = test_register(&mut client, "密码管理插件", "密码管理插件").await;
        
        // 使用简化版注册返回的ID或默认ID测试心跳
        let id_for_heartbeat = if simple_plugin_id.is_empty() { "password-plugin" } else { &simple_plugin_id };
        test_heartbeat(&mut client, id_for_heartbeat).await;
    } else {
        // 使用标准注册返回的ID测试心跳
        test_heartbeat(&mut client, &plugin_id).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置服务器地址
    let server_host = "localhost";
    let server_port = 19090;
    
    // 只使用 http 协议，因为这是最常见的
    let endpoint_str = format!("http://{}:{}", server_host, server_port);
    
    println!("测试连接到gRPC服务器: {}", endpoint_str);
    
    // 首先使用TCP直接连接测试
    let tcp_success = test_tcp_connection(server_host, server_port).await;
    
    if !tcp_success {
        println!("TCP连接失败，服务器可能未启动或端口未开放");
        return Ok(());
    }
    
    println!("尝试使用 {} 连接...", endpoint_str);
    
    // 创建一个更简单的端点配置，减少可能的兼容性问题
    let channel = match Endpoint::from_shared(endpoint_str.clone()) {
        Ok(endpoint) => {
            // 使用更接近Java客户端的配置，但移除不支持的方法
            let endpoint = endpoint
                .connect_timeout(Duration::from_secs(30))
                .timeout(Duration::from_secs(30))
                .keep_alive_while_idle(true)
                .http2_keep_alive_interval(Duration::from_secs(20));
            // 移除不支持的 http2_prior_knowledge 方法
        
            // 连接并等待结果
            match endpoint.connect().await {
                Ok(channel) => {
                    println!("gRPC连接成功!");
                    Some(channel)
                },
                Err(e) => {
                    println!("gRPC连接失败: {}", e);
                    println!("错误详情: {:?}", e);
                    None
                }
            }
        },
        Err(e) => {
            println!("无效的端点格式: {}", e);
            None
        }
    };
    
    // 如果连接成功，尝试调用各种方法
    if let Some(channel) = channel {
        let mut client = PluginServiceClient::new(channel);
        
        // 先尝试心跳，因为这个已知可以工作
        println!("\n测试心跳功能...");
        test_heartbeat(&mut client, "test-plugin").await;
        
        // 然后尝试注册
        println!("\n测试注册功能...");
        let plugin_id = test_register(&mut client, "密码管理插件", "密码管理插件").await;
        
        if !plugin_id.is_empty() {
            println!("注册成功，使用注册返回的ID测试心跳");
            test_heartbeat(&mut client, &plugin_id).await;
        } else {
            println!("注册失败，使用默认ID继续测试心跳");
            test_heartbeat(&mut client, "password-plugin").await;
        }
    } else {
        println!("无法建立gRPC连接，请检查服务器配置");
    }
    
    Ok(())
}


