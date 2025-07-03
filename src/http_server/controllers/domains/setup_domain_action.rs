use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/domain/setup",
    description: "Setup domain mask. Please use the following format: *-env_name.domain.com",
    summary: "Setup domain mask",
    input_data: SetupDomainHttpRequest,
    controller: "Domains",

    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]
pub struct SetupDomainAction {
    app: Arc<AppContext>,
}

impl SetupDomainAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SetupDomainAction,
    input_data: SetupDomainHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::set_domain_mask(&action.app, &input_data.domain_mask).await;

    HttpOutput::Empty.into_ok_result(false)
}
