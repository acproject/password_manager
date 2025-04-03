pub mod base_plugin;
pub mod command_result;
pub mod example_plugin;
pub mod key_management;  // 新的模块
pub mod persistence;
pub mod plugin_config;
pub mod plugin_info;
pub mod plugin_sdk;

pub use base_plugin::BasePlugin;
pub use command_result::CommandResult;
pub use example_plugin::ExamplePlugin;
pub use key_management::KeyManagementPlugin;  // 从新模块导出
pub use plugin_config::PluginConfig;
pub use plugin_info::PluginInfo;
pub use plugin_sdk::PluginSDK;