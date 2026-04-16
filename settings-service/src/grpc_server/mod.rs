use std::sync::Arc;

use crate::app_ctx::AppContext;

mod secrets_grpc_service;
mod templates_grpc_service;

#[derive(Clone)]
pub struct SdkGrpcService {
    pub app: Arc<AppContext>,
}

impl SdkGrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

pub async fn start(app: Arc<AppContext>) {
    let port = app.settings.get_grpc_port();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let service = SdkGrpcService::new(app);

    println!("Listening to Tcp({:?}) as grpc endpoint", addr);

    tonic::transport::Server::builder()
        .add_service(crate::secrets_grpc::secrets_server::SecretsServer::new(
            service.clone(),
        ))
        .add_service(crate::templates_grpc::templates_server::TemplatesServer::new(service))
        .serve(addr)
        .await
        .expect("gRPC server failed");
}
