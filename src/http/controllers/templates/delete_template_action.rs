use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/templates/delete",
    description: "Delete template",
    summary: "Deletes template",
    controller: "Templates",
    input_data: "DeleteTemplateContract",

    result:[
        {status_code: 202, description: "Ok response"},
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
    input_data: DeleteTemplateContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::operations::templates::delete(&action.app, input_data.env, input_data.name).await;
    HttpOutput::Empty.into_ok_result(false)
}
