use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/usageBySecrets",
    description: "Get secret",
    summary: "Returns secret",
    controller: "Secrets",
    input_data: ShowSecretesUsageInputContract,
    result:[
        {status_code: 200, description: "Ok response", model="Vec<SecretSecretUsageHttpModel>"},
    ]
)]
pub struct ShowUsageBySecretsAction {
    app: Arc<AppContext>,
}

impl ShowUsageBySecretsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ShowUsageBySecretsAction,
    input_data: ShowSecretesUsageInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_secrets_used_by_the_secret(
        &action.app,
        input_data.product.as_deref().into(),
        &input_data.name,
    )
    .await;

    let result: Vec<SecretSecretUsageHttpModel> = result
        .into_iter()
        .map(|itm| SecretSecretUsageHttpModel {
            name: itm.id,
            value: itm.value,
        })
        .collect();

    HttpOutput::as_json(result).into_ok_result(false)
}
