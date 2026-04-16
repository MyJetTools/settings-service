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
    input_data: GetSecretContract,
    result:[
        {status_code: 200, description: "Ok response", model: "ShowSecretHttpResponse"},
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

    let secrets_snapshot = action.app.secrets.get_snapshot().await;
    let secret_result = secrets_snapshot.get_by_id(product_id, &input_data.name);

    let Some(secret_item) = secret_result else {
        return HttpOutput::as_not_found("Secret not found").into_err(false, false);
    };

    let value = crate::scripts::populate_secrets(
        action.app.as_ref(),
        product_id,
        &secret_item.content,
        &secrets_snapshot,
        0,
        false,
    );

    let remote_value = secret_item.remote_value.as_ref().map(|remote| {
        crate::scripts::populate_secrets(
            action.app.as_ref(),
            product_id,
            remote,
            &secrets_snapshot,
            0,
            true,
        )
        .into_string()
    });

    let response = ShowSecretHttpResponse {
        value: value.into_string(),
        remote_value,
    };

    HttpOutput::as_json(response).into_ok_result(false)
}
