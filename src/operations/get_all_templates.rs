use std::sync::Arc;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn get_all_templates(app: &AppContext) -> Vec<Arc<TemplateMyNoSqlEntity>> {
    super::initialize_templates(app).await;
    app.templates_cache.get_all().await
}
