use std::time::Duration;

use crate::app_ctx::AppContext;

pub async fn delete(app: &AppContext, evn: String, name: String) {
    app.templates_storage
        .delete_row(evn.as_str(), name.as_str())
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;
}
