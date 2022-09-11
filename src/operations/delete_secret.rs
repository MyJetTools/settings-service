use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn delete_secret(app: &AppContext, secret_name: &str) {
    app.key_vault_client
        .delete_secret(secret_name)
        .await
        .unwrap();

    app.secrets_storage
        .delete_row(SecretMyNoSqlEntity::generate_partition_key(), secret_name)
        .await
        .unwrap();

    app.secrets_cache.remove(secret_name).await;
}
