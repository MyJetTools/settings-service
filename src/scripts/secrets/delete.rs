use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn delete(app: &AppContext, secret_name: &str) {
    app.secrets_storage
        .delete_row(SecretMyNoSqlEntity::generate_partition_key(), secret_name)
        .await
        .unwrap();
}
