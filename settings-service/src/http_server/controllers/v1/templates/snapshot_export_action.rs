use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/v1/templates/snapshot/export",
    description: "Export snapshot (templates + secrets) for a product as downloadable JSON file",
    summary: "Export snapshot",
    controller: "V1::Templates",
    input_data: SnapshotExportInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SnapshotExportAction {
    app: Arc<AppContext>,
}

impl SnapshotExportAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SnapshotExportAction,
    input_data: SnapshotExportInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let templates_only = input_data.templates_only.unwrap_or(false);

    let export_data =
        crate::flows::export_snapshot(&action.app, &input_data.product_id, templates_only).await;

    let dt = DateTimeAsMicroseconds::now()
        .to_compact_date_time_string();

    HttpOutput::File {
        file_name: format!("settings_snapshot_{dt}.json"),
        content: export_data.to_string().into_bytes(),
        headers: Default::default(),
    }
    .into_ok_result(false)
}
