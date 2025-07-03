use std::sync::Arc;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn get(app: &AppContext, env: &str, name: &str) -> Option<Arc<TemplateMyNoSqlEntity>> {
    app.templates_storage_reader.get_entity(env, name).await
}
