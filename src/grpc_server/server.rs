use crate::app_ctx::AppContext;
use crate::templates_grpc::templates_server::TemplatesServer;

use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;

#[derive(Clone)]

pub struct GrpcService {
    pub app: Arc<AppContext>,
}

impl GrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

pub async fn start(app: Arc<AppContext>, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = GrpcService::new(app);

    println!("Listening to {:?} as grpc endpoint", addr);

    anyhow::Context::context(
        Server::builder()
            .add_service(TemplatesServer::new(service.clone()))
            .serve(addr)
            .await,
        "Server error",
    )
    .unwrap();
}
