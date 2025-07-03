use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/dump/snapshot/export",
    description: "Export Templates and Secrets",
    summary: "Export Templates and Secrets",
    controller: "Dump",
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]

pub struct ExportAction {
    app: Arc<AppContext>,
}

impl ExportAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ExportAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let max_level = action.app.settings.max_level_of_secrets_to_export;
    let content = crate::flows::export_snapshot(&action.app, max_level).await;

    let dt = DateTimeAsMicroseconds::now();

    let dt = dt.to_compact_date_time_string();
    HttpOutput::File {
        file_name: format!("settings_snapshot_{dt}.json"),
        content,
    }
    .into_ok_result(false)
}
