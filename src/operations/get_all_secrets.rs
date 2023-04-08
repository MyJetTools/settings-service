use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn get_all_secrets(app: &AppContext) -> Option<Vec<SecretMyNoSqlEntity>> {
    app.secrets_repository.get_all().await
}
