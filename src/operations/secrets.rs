use crate::app_ctx::AppContext;

pub async fn get_used_secret_amount(app: &AppContext, secret_name: &str) -> usize {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    let secret_name = format!("${{{}}}", secret_name);

    app.templates_cache
        .get_used_secret_amount(secret_name.as_str())
        .await
}
