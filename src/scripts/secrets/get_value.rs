use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

use crate::models::*;

pub async fn get_value(app: &AppContext, secret_name: &str) -> Option<SecretValue> {
    let entity = app
        .secrets_storage
        .get_entity(
            SecretMyNoSqlEntity::generate_partition_key(),
            secret_name,
            None,
        )
        .await
        .unwrap()?;

    return super::decode(&entity, &app.aes_key);
}
