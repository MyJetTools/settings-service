use crate::app_ctx::AppContext;

pub async fn get(app: &AppContext, secret_name: &str) -> Option<String> {
    if !app.secrets_cache.is_initialized() {
        let secrets = app.secrets_storage.get_all().await.unwrap();
        app.secrets_cache.init(secrets).await;
    }

    if !app.secrets_cache.has_value(secret_name).await {
        return None;
    }

    let result = app.secrets_values_cache.get(secret_name).await;

    if result.is_some() {
        return result;
    }

    let result = app.key_vault_client.get_secret(secret_name).await.unwrap();

    if let Some(result) = result.as_ref() {
        app.secrets_values_cache
            .set(secret_name.to_string(), result.clone())
            .await;
    }

    result
}
