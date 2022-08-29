use std::sync::Arc;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn get_all(app: &AppContext) -> Vec<Arc<TemplateMyNoSqlEntity>> {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    app.templates_cache.get_all().await
}

pub async fn get(app: &AppContext, evn: &str, name: &str) -> Option<Arc<TemplateMyNoSqlEntity>> {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    app.templates_cache.get(evn, name).await
}
