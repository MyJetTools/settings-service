use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

use crate::models::*;

pub async fn get_value(
    app: &AppContext,
    env: Option<&str>,
    secret_name: &str,
) -> Option<SecretValue> {
    let partition_key = env.unwrap_or(SecretMyNoSqlEntity::DEFAULT_PARTITION_KEY);
    let entity = app
        .secrets_storage
        .get_entity(partition_key, secret_name, None)
        .await
        .unwrap()?;

    return super::decode(&entity, &app.aes_key);
}
