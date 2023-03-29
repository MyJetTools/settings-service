use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
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
    let template =
        crate::operations::templates::get(&action.app, &input_data.env, &input_data.name).await;

    if let Some(template) = template {
        HttpOutput::as_text(template.yaml_template.clone()).into_ok_result(false)
    } else {
        Err(HttpFailResult::as_not_found("Not Found".to_string(), false))
    }
}
