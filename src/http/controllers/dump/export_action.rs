use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

use super::contracts::ExportInputModel;

#[http_route(
    method: "GET",
    route: "/api/dump/export/templates",
    description: "Export Templates and Secrets",
    summary: "Export Templates and Secrets",
    input_data: ExportInputModel,
    controller: "Dump",
    result:[
        {status_code: 202, description: "Ok response"},
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
    input_data: ExportInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content = crate::operations::export_to_zip(&action.app, input_data.secrets_max_level).await;

    let dt = DateTimeAsMicroseconds::now();

    let dt = dt.to_compact_date_time_string();
    HttpOutput::File {
        file_name: format!("settings_snapshot_{dt}.zip"),
        content,
    }
    .into_ok_result(false)
}
