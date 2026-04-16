use std::sync::Arc;

use my_http_server::{macros::*, *};

use crate::app_ctx::AppContext;

use super::contracts::ImportSettingsTemplateAction;

#[http_route(
    method: "POST",
    route: "/api/dump/import/templates",
    description: "Import Templates",
    summary: "Import Templates",
    controller: "Dump",
    input_data: "ImportSettingsTemplateAction",
    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]

pub struct ImportTemplatesAction {
    app: Arc<AppContext>,
}

impl ImportTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ImportTemplatesAction,
    input_data: ImportSettingsTemplateAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::import_snapshot(
        &action.app,
        &input_data.product,
        &input_data.dump.content,
        true,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
