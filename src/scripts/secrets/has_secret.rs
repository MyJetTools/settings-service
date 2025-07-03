use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn has_secret(app: &AppContext, secret_name: &str) -> bool {
    let result = app
        .secrets_storage
        .get_entity(
            SecretMyNoSqlEntity::generate_partition_key(),
            secret_name,
            None,
        )
        .await
        .unwrap();

    result.is_some()
}
