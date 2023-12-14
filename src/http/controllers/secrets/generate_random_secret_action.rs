use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, caches::SecretValue};

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
    let secret_name = input_data.name.to_string();
    let secret_value: SecretValue = input_data.into();

    let result = secret_value.content.to_string();
    crate::operations::update_secret(&action.app, secret_name, secret_value).await;

    HttpOutput::as_text(result).into_ok_result(false)
}
