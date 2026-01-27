use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/templates/get",
    description: "Get template",
    summary: "Returns template",
    controller: "Templates",
    input_data: "GetTemplateContract",

    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct GetTemplateAction {
    app: Arc<AppContext>,
}

impl GetTemplateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetTemplateAction,
    input_data: GetTemplateContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let template_content = action
        .app
        .templates
        .get_by_id(&input_data.product, &input_data.name, |itm| {
            itm.content.clone()
        })
        .await;

    let Some(template_content) = template_content else {
        return Err(HttpFailResult::as_not_found("Not Found".to_string(), false));
    };

    HttpOutput::as_text(template_content.into_string()).into_ok_result(false)
}
