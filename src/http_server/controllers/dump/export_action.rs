use std::sync::Arc;

use my_http_server::{macros::*, *};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/dump/snapshot/export",
    description: "Export Templates and Secrets",
    summary: "Export Templates and Secrets",
    controller: "Dump",
    input_data: ExportSnapshotHttpInputData,
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
    input_data: ExportSnapshotHttpInputData,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let export_data = crate::flows::export_snapshot(&action.app, &input_data.product, false).await;

    let dt = DateTimeAsMicroseconds::now();

    let dt = dt.to_compact_date_time_string();
    HttpOutput::File {
        file_name: format!("settings_snapshot_{dt}.json"),
        content: export_data.to_string().into_bytes(),
    }
    .into_ok_result(false)
}
