use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "GET",
    route: "/api/v1/secrets/usage/by-secrets",
    description: "List other secrets that reference the secret in their value",
    summary: "Secret usage by secrets",
    controller: "V1::Secrets",
    input_data: SecretUsageInput,
    result: [
        {status_code: 200, description: "Array of SecretUsageBySecretHttpModel"},
    ]
)]
pub struct UsageBySecretsAction {
    app: Arc<AppContext>,
}

impl UsageBySecretsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &UsageBySecretsAction,
    input_data: SecretUsageInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product_id: ProductId = input_data.product_id.as_deref().into();

    let usage = crate::flows::get_secrets_used_by_the_secret(
        &action.app,
        product_id,
        &input_data.secret_id,
    )
    .await;

    let result: Vec<SecretUsageBySecretHttpModel> = usage
        .into_iter()
        .map(|itm| SecretUsageBySecretHttpModel {
            product_id: if itm.product_id.is_empty() {
                None
            } else {
                Some(itm.product_id)
            },
            secret_id: itm.id,
            value: itm.value,
        })
        .collect();

    HttpOutput::as_json(result).into_ok_result(false)
}
