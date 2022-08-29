use std::sync::Arc;

use crate::settings_model::SettingsModel;

mod app_ctx;
mod caches;
mod env_settings;
mod http;
mod my_no_sql;
mod operations;
mod settings_model;
#[tokio::main]
async fn main() {
    let settings = SettingsModel::load(".settings-service").await;

    let app = crate::app_ctx::AppContext::new(settings);

    let app = Arc::new(app);

    crate::http::start(&app);

    app.app_states.wait_until_shutdown().await;
}
