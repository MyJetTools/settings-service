use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/templates/post",
    description: "Add or update template",
    controller: "Templates",
    input_data: "PostTemplateContract",

    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct PostTemplateAction {
    app: Arc<AppContext>,
}

impl PostTemplateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostTemplateAction,
    input_data: PostTemplateContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::operations::templates::post(
        &action.app,
        input_data.env,
        input_data.name,
        input_data.yaml,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
