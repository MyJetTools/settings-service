use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/v1/templates/content",
    description: "Get template content (raw yaml)",
    summary: "Get template content",
    controller: "V1::Templates",
    input_data: GetTemplateContentInput,
    result: [
        {status_code: 200, description: "Ok response", model: "TemplateContentHttpModel"},
        {status_code: 404, description: "Template not found"},
    ]
)]
pub struct GetTemplateContentAction {
    app: Arc<AppContext>,
}

impl GetTemplateContentAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetTemplateContentAction,
    input_data: GetTemplateContentInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content = action
        .app
        .templates
        .get_by_id(&input_data.product_id, &input_data.template_id, |itm| {
            itm.content.to_string()
        })
        .await;

    let Some(content) = content else {
        return Err(HttpFailResult::as_not_found(
            "Template not found".to_string(),
            false,
        ));
    };

    HttpOutput::as_json(TemplateContentHttpModel { content }).into_ok_result(false)
}
