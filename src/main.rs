use std::sync::Arc;

use crate::settings::SettingsModel;

mod app_ctx;
mod caches;

mod grpc_server;
mod http_server;
//mod key_value_repository;

mod flows;
mod models;
//mod my_no_sql;
mod secret_generator;
mod settings;

mod consts;
mod mappers;
mod persistence;
mod scripts;

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
    let settings = SettingsModel::first_load("~/.settings-service")
        .await
        .into();

    let app = crate::app_ctx::AppContext::new(settings).await;

    let app = Arc::new(app);

    crate::scripts::init(&app).await;

    crate::http_server::start(&app);

    tokio::spawn(crate::grpc_server::server::start(app.clone(), 8888));

    app.app_states.wait_until_shutdown().await;
}
