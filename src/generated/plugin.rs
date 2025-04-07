// @generated
// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPluginByNameRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPluginByNameResponse {
    #[prost(bool, tag="1")]
    pub found: bool,
    #[prost(message, optional, tag="2")]
    pub plugin: ::core::option::Option<PluginInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindPluginRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindPluginResponse {
    #[prost(bool, tag="1")]
    pub found: bool,
    #[prost(message, optional, tag="2")]
    pub plugin: ::core::option::Option<PluginInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfo {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag="7")]
    pub port: i32,
    #[prost(string, tag="8")]
    pub status: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePluginRequest {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePluginResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// 插件注册请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginRegistration {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub port: i32,
}
/// 注册响应
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResponse {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub success: bool,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
/// 心跳请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRequest {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status_info: ::prost::alloc::string::String,
}
/// 心跳响应
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HeartbeatResponse {
    #[prost(bool, tag="1")]
    pub received: bool,
    #[prost(int64, tag="2")]
    pub server_time: i64,
}
/// 状态请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
}
/// 状态响应
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub details: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub uptime: i64,
}
/// 命令请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub command: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// 命令响应
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub result: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub error_message: ::prost::alloc::string::String,
}
/// 停止请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    #[prost(string, tag="1")]
    pub plugin_id: ::prost::alloc::string::String,
}
/// 停止响应
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
include!("plugin.tonic.rs");
// @@protoc_insertion_point(module)
