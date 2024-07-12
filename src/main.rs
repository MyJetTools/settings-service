use std::sync::Arc;

use crate::settings_model::SettingsModel;

mod app_ctx;
mod caches;
mod env_settings;
mod grpc_server;
mod http_server;
mod key_value_repository;
mod my_no_sql;
mod operations;
mod secret_generator;
mod settings_model;
#[allow(non_snake_case)]
pub mod domains_grpc {
    tonic::include_proto!("domains");
}

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

    let app = crate::app_ctx::AppContext::new(settings).await;

    let app = Arc::new(app);

    crate::http_server::start(&app);

    tokio::spawn(crate::grpc_server::server::start(app.clone(), 8888));

    if let Some(init_from_file) = app.settings.init_from_file.as_ref() {
        crate::operations::init_on_start(&app, init_from_file).await;
    } else {
        println!("Settings InitFromFile is not set. Skipping initialization settings from file.");
    }

    app.app_states.wait_until_shutdown().await;
}
