use async_trait::async_trait;
use std::collections::HashMap;

use crate::base_plugin::BasePlugin;
use crate::command_result::CommandResult;
use crate::plugin_config::PluginConfig;
use crate::plugin_sdk::PluginSDK;

/// 示例插件实现
pub struct ExamplePlugin {
    base: BasePlugin,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            base: BasePlugin::new(),
        }
    }
}

#[async_trait]
impl PluginSDK for ExamplePlugin {
    async fn initialize(&mut self, config: PluginConfig) -> bool {
        let result = self.base.initialize(config).await;
        
        // 设置插件详细信息，确保与后端SysPlugin.java匹配
        let mut info = self.base.get_info();
        info.set_name("密码管理示例插件".to_string());
        info.set_version("1.0.0".to_string());
        info.set_type("PASSWORD_MANAGER".to_string()); // 确保类型与后端期望的一致
        info.set_description("密码管理器示例插件，提供基本的密码管理功能".to_string());
        info.set_status("READY".to_string()); // 设置初始状态
        
        // 添加支持的命令
        info.add_supported_command("hello".to_string());
        info.add_supported_command("echo".to_string());
        info.add_supported_command("get_password".to_string());
        info.add_supported_command("save_password".to_string());
        
        // 添加支持的事件
        info.add_supported_event("startup".to_string());
        info.add_supported_event("password_changed".to_string());
        
        // 尝试注册插件
        match self.base.retry_register().await {
            Ok(_) => println!("插件注册成功"),
            Err(e) => eprintln!("插件注册失败: {}", e),
        }
        
        result
    }

    async fn start(&mut self) -> bool {
        println!("启动密码管理示例插件...");
        
        // 更新插件状态为运行中
        let mut info = self.base.get_info();
        info.set_status("RUNNING".to_string());
        
        self.base.start().await
    }

    async fn stop(&mut self) -> bool {
        println!("停止密码管理示例插件...");
        
        // 更新插件状态为已停止
        let mut info = self.base.get_info();
        info.set_status("STOPPED".to_string());
        
        self.base.stop().await
    }

    fn get_info(&self) -> crate::plugin_info::PluginInfo {
        self.base.get_info()
    }

    async fn execute_command(&self, command: &str, params: &HashMap<String, String>) -> CommandResult {
        println!("执行命令: {}", command);
        
        match command {
            "hello" => CommandResult::new(
                true,
                "Hello, World!".to_string(),
                String::new(),
            ),
            "echo" => {
                let message = params.get("message").cloned().unwrap_or_default();
                CommandResult::new(
                    true,
                    format!("Echo: {}", message),
                    String::new(),
                )
            },
            "get_password" => {
                let username = params.get("username").cloned().unwrap_or_default();
                let service = params.get("service").cloned().unwrap_or_default();
                
                // 这里应该实现实际的密码获取逻辑
                CommandResult::new(
                    true,
                    format!("用户 {} 在服务 {} 的密码是: ********", username, service),
                    String::new(),
                )
            },
            "save_password" => {
                let username = params.get("username").cloned().unwrap_or_default();
                let service = params.get("service").cloned().unwrap_or_default();
                
                // 这里应该实现实际的密码保存逻辑
                CommandResult::new(
                    true,
                    format!("已保存用户 {} 在服务 {} 的密码", username, service),
                    String::new(),
                )
            },
            _ => CommandResult::new(
                false,
                String::new(),
                format!("未知命令: {}", command),
            ),
        }
    }

    async fn handle_message(&self, message: &str) -> String {
        println!("处理消息: {}", message);
        format!("已收到消息: {}", message)
    }
}