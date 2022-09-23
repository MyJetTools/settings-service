use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/secrets/get",
    description: "Get secret",
    controller: "Secrets",
    input_data: "GetSecretContract",
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct GetSecretAction {
    app: Arc<AppContext>,
}

impl GetSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetSecretAction,
    input_data: GetSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = action
        .app
        .key_value_repository
        .get_secret(input_data.name.as_str())
        .await;

    match result {
        Some(result) => HttpOutput::as_text(result).into_ok_result(false),
        None => Err(HttpFailResult::as_not_found(
            "Secret not found".to_string(),
            false,
        )),
    }
}
