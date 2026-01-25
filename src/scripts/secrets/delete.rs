use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn delete(app: &AppContext, env: Option<&str>, secret_name: &str) {
    let partition_key = env.unwrap_or(SecretMyNoSqlEntity::DEFAULT_PARTITION_KEY);
    app.secrets_storage
        .delete_row(partition_key, secret_name)
        .await
        .unwrap();
}
