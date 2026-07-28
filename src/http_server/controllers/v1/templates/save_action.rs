use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/templates",
    description: "Save (create or update) a template",
    summary: "Save template",
    controller: "V1::Templates",
    input_data: SaveTemplateInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SaveTemplateAction {
    app: Arc<AppContext>,
}

impl SaveTemplateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SaveTemplateAction,
    input_data: SaveTemplateInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::save_template(
        &action.app,
        &input_data.product_id,
        input_data.template_id,
        input_data.yaml,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
