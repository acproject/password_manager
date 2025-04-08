// @generated
/// Generated client implementations.
pub mod plugin_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PluginServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PluginServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PluginServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PluginServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            PluginServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn register_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginRegistration>,
        ) -> std::result::Result<
            tonic::Response<super::RegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/RegisterPlugin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "RegisterPlugin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartbeatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeartbeatResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/Heartbeat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "Heartbeat"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/GetStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "GetStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_command(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CommandResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/ExecuteCommand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "ExecuteCommand"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/StopPlugin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "StopPlugin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn find_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::FindPluginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindPluginResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/FindPlugin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "FindPlugin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePluginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePluginResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/UpdatePlugin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "UpdatePlugin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_plugin_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPluginByNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPluginByNameResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plugin.PluginService/GetPluginByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("plugin.PluginService", "GetPluginByName"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod plugin_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PluginServiceServer.
    #[async_trait]
    pub trait PluginService: std::marker::Send + std::marker::Sync + 'static {
        async fn register_plugin(
            &self,
            request: tonic::Request<super::PluginRegistration>,
        ) -> std::result::Result<
            tonic::Response<super::RegistrationResponse>,
            tonic::Status,
        >;
        async fn heartbeat(
            &self,
            request: tonic::Request<super::HeartbeatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeartbeatResponse>,
            tonic::Status,
        >;
        async fn get_status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        async fn execute_command(
            &self,
            request: tonic::Request<super::CommandRequest>,
        ) -> std::result::Result<tonic::Response<super::CommandResponse>, tonic::Status>;
        async fn stop_plugin(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status>;
        async fn find_plugin(
            &self,
            request: tonic::Request<super::FindPluginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindPluginResponse>,
            tonic::Status,
        >;
        async fn update_plugin(
            &self,
            request: tonic::Request<super::UpdatePluginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePluginResponse>,
            tonic::Status,
        >;
        async fn get_plugin_by_name(
            &self,
            request: tonic::Request<super::GetPluginByNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPluginByNameResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PluginServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PluginServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PluginServiceServer<T>
    where
        T: PluginService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/plugin.PluginService/RegisterPlugin" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterPluginSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::PluginRegistration>
                    for RegisterPluginSvc<T> {
                        type Response = super::RegistrationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PluginRegistration>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::register_plugin(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterPluginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/Heartbeat" => {
                    #[allow(non_camel_case_types)]
                    struct HeartbeatSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::HeartbeatRequest>
                    for HeartbeatSvc<T> {
                        type Response = super::HeartbeatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeartbeatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::heartbeat(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = HeartbeatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::StatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::StatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::get_status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/ExecuteCommand" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteCommandSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::CommandRequest>
                    for ExecuteCommandSvc<T> {
                        type Response = super::CommandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::execute_command(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExecuteCommandSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/StopPlugin" => {
                    #[allow(non_camel_case_types)]
                    struct StopPluginSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::StopRequest>
                    for StopPluginSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::stop_plugin(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopPluginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/FindPlugin" => {
                    #[allow(non_camel_case_types)]
                    struct FindPluginSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::FindPluginRequest>
                    for FindPluginSvc<T> {
                        type Response = super::FindPluginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindPluginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::find_plugin(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FindPluginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/UpdatePlugin" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePluginSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::UpdatePluginRequest>
                    for UpdatePluginSvc<T> {
                        type Response = super::UpdatePluginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePluginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::update_plugin(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdatePluginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plugin.PluginService/GetPluginByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetPluginByNameSvc<T: PluginService>(pub Arc<T>);
                    impl<
                        T: PluginService,
                    > tonic::server::UnaryService<super::GetPluginByNameRequest>
                    for GetPluginByNameSvc<T> {
                        type Response = super::GetPluginByNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPluginByNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PluginService>::get_plugin_by_name(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetPluginByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for PluginServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "plugin.PluginService";
    impl<T> tonic::server::NamedService for PluginServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
