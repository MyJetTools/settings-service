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
