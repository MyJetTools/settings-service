use crate::{app_ctx::AppContext, caches::SecretUsage};

pub async fn show_secret_usage(app: &AppContext, secret_name: &str) -> Vec<SecretUsage> {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    let secret_name = format!("${{{}}}", secret_name);

    app.templates_cache
        .get_secret_usages(secret_name.as_str())
        .await
}
