use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn has_secret(app: &AppContext, env: Option<&str>, secret_name: &str) -> bool {
    let partition_key = env.unwrap_or(SecretMyNoSqlEntity::DEFAULT_PARTITION_KEY);
    let result = app
        .secrets_storage
        .get_entity(partition_key, secret_name, None)
        .await
        .unwrap();

    result.is_some()
}
