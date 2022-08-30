use std::sync::Arc;

use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn get_all(app: &AppContext) -> Vec<Arc<SecretMyNoSqlEntity>> {
    if !app.secrets_cache.is_initialized() {
        let secrets = app
            .secrets_storage
            .get_by_partition_key(SecretMyNoSqlEntity::generate_partition_key())
            .await
            .unwrap();
        app.secrets_cache.init(secrets).await;
    }

    app.secrets_cache.get_all().await
}

pub async fn get_used_secret_amount(app: &AppContext, secret_name: &str) -> usize {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    let secret_name = format!("${{{}}}", secret_name);

    app.templates_cache
        .get_used_secret_amount(secret_name.as_str())
        .await
}
