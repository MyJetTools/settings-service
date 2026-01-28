use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/usage",
    description: "Get secret",
    summary: "Returns secret",
    controller: "Secrets",
    input_data: "ShowUsageInputContract",
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct ShowUsageAction {
    app: Arc<AppContext>,
}

impl ShowUsageAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ShowUsageAction,
    input_data: ShowUsageInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_templates_used_by_the_secret(
        &action.app,
        input_data.product.as_deref().into(),
        &input_data.secret,
    )
    .await;

    let response = ShowSecretUsageHttpResponse::new(result);

    HttpOutput::as_json(response).into_ok_result(false)
}
