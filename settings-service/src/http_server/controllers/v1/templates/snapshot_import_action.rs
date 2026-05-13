use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/templates/snapshot/import",
    description: "Import a snapshot (templates + secrets) for a product",
    summary: "Import snapshot",
    controller: "V1::Templates",
    input_data: SnapshotImportInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SnapshotImportAction {
    app: Arc<AppContext>,
}

impl SnapshotImportAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SnapshotImportAction,
    input_data: SnapshotImportInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::import_snapshot(
        &action.app,
        &input_data.product_id,
        &input_data.snapshot.content,
        false,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
