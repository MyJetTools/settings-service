use crate::app_ctx::AppContext;

pub async fn delete_secret(app: &AppContext, secret_name: &str) {
    app.secrets_repository.delete_secret(secret_name).await;
    app.secret_values_cache.delete(&secret_name).await;
}
