use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/generate",
    description: "Generate random secret",
    summary: "Generate random secret",
    controller: "Secrets",
    input_data: GenerateRandomSecretContract,
    result:[
        {status_code: 200, description: "Generated Secret"},
    ]
)]
pub struct GenerateRandomSecretAction {
    app: Arc<AppContext>,
}

impl GenerateRandomSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GenerateRandomSecretAction,
    input_data: GenerateRandomSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::operations::update_secret(&action.app, input_data.name.to_string(), input_data.into())
        .await;

    HttpOutput::Empty.into_ok_result(false)
}
