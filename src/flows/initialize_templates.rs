use crate::app_ctx::AppContext;

pub async fn initialize_templates(app: &AppContext, force_initialize: bool) {
    if !force_initialize && app.templates_cache.is_initialized() {
        return;
    }
    let templates = app.templates_storage.get_all().await.unwrap();
    app.templates_cache.init(templates).await;
}
