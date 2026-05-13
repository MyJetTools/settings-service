use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/templates/delete",
    description: "Delete a template",
    summary: "Delete template",
    controller: "V1::Templates",
    input_data: DeleteTemplateInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct DeleteTemplateAction {
    app: Arc<AppContext>,
}

impl DeleteTemplateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteTemplateAction,
    input_data: DeleteTemplateInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::delete_template(&action.app, &input_data.product_id, &input_data.template_id)
        .await;

    HttpOutput::Empty.into_ok_result(false)
}
