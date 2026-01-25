use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/get",
    description: "Get secret",
    summary: "Returns secret",
    controller: "Secrets",
    input_data: "GetSecretContract",
    result:[
        {status_code: 200, description: "Ok response", model: "SecretHttpModel"},
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
    let result = crate::scripts::secrets::get_value(
        &action.app,
        input_data.env.as_deref(),
        &input_data.name,
    )
    .await;

    match result {
        Some(result) => {
            let model: SecretHttpModel = result.into();
            HttpOutput::as_json(model).into_ok_result(false)
        }
        None => Err(HttpFailResult::as_not_found(
            "Secret not found".to_string(),
            false,
        )),
    }
}
