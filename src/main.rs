use std::sync::Arc;

use crate::settings_model::SettingsModel;

mod app_ctx;
mod caches;
mod env_settings;
mod grpc_server;
mod http;
mod key_value_repository;
mod my_no_sql;
mod operations;
mod settings_model;

#[allow(non_snake_case)]
pub mod templates_grpc {
    tonic::include_proto!("templates");
}

#[allow(non_snake_case)]
pub mod secrets_grpc {
    tonic::include_proto!("secrets");
}

#[tokio::main]
async fn main() {
    let settings = SettingsModel::first_load(".settings-service").await.into();

    let app = crate::app_ctx::AppContext::new(settings);

    let app = Arc::new(app);

    crate::http::start(&app);

    tokio::spawn(crate::grpc_server::server::start(app.clone(), 8888));

    app.app_states.wait_until_shutdown().await;
}
