use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::*;

#[http_route(
    method: "POST",
    route: "/api/secrets/show",
    description: "Shows secret with resolved references",
    summary: "Show secret",
    controller: "Secrets",
    input_data: "GetSecretContract",
    result:[
        {status_code: 200, description: "Ok response", model: "SecretHttpModel"},
    ]
)]
pub struct ShowSecretAction {
    app: Arc<AppContext>,
}

impl ShowSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ShowSecretAction,
    input_data: GetSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product_id = input_data.product.as_deref().into();

    let secrets = action.app.secrets.get_snapshot().await;
    let secret_result = secrets.get_by_id(product_id, &input_data.name);

    match secret_result {
        Some(secret_result) => {
            let secrets_snapshot = action.app.secrets.get_snapshot().await;
            let result = crate::scripts::populate_secrets(
                action.app.as_ref(),
                product_id,
                &secret_result.content,
                &secrets_snapshot,
                0,
            );
            return HttpOutput::as_text(result.into_string()).into_ok_result(false);
        }
        None => HttpOutput::as_not_found("Secret not found").into_err(false, false),
    }
}
