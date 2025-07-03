use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/dump/snapshot/import",
    description: "Import Templates and Secrets",
    summary: "Import Templates and Secrets",
    input_data: ImportSnapshotModel,
    controller: "Dump",
    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]

pub struct ImportAction {
    app: Arc<AppContext>,
}

impl ImportAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ImportAction,
    input_data: ImportSnapshotModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::import_snapshot(&action.app, &input_data.dump.content).await;
    HttpOutput::Empty.into_ok_result(false)
}
