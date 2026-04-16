use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/templates/getall",
    description: "Get list of templates",
    summary: "Returns list of templates",
    controller: "Templates",

    result:[
        {status_code: 200, description: "Ok response", model: "ListOfTemplatesContract"},
    ]
)]
pub struct GetTemplatesAction {
    app: Arc<AppContext>,
}

impl GetTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetTemplatesAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let templates = crate::flows::get_all_templates(&action.app).await;

    let result = ListOfTemplatesContract::new(templates);
    HttpOutput::as_json(result).into_ok_result(false)
}
