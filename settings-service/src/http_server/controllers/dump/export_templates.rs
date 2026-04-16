use std::sync::Arc;

use my_http_server::{macros::*, *};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

use super::contracts::*;
#[http_route(
    method: "GET",
    route: "/api/dump/export/templates",
    description: "Export Templates",
    summary: "Export Templates",
  input_data: ExportSnapshotHttpInputData,
    controller: "Dump",
    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]

pub struct ExportTemplatesAction {
    app: Arc<AppContext>,
}

impl ExportTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ExportTemplatesAction,
    input_data: ExportSnapshotHttpInputData,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let export_data = crate::flows::export_snapshot(&action.app, &input_data.product, true).await;

    let dt = DateTimeAsMicroseconds::now();

    let dt = dt.to_compact_date_time_string();
    HttpOutput::File {
        file_name: format!("settings_snapshot_{dt}.json"),
        content: export_data.to_string().into_bytes(),
    }
    .into_ok_result(false)
}
