use std::sync::Arc;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn get_all(app: &AppContext) -> Vec<Arc<TemplateMyNoSqlEntity>> {
    app.templates_storage_reader
        .get_table_snapshot_as_vec()
        .await
        .unwrap_or_default()
}
