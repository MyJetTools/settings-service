use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

use crate::models::*;

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
    mut input_data: GenerateRandomSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product = std::mem::take(&mut input_data.product);
    let product_id: ProductId = product.as_deref().into();

    let secrets = action.app.secrets.get_snapshot().await;

    if !input_data.has_force_update() {
        if secrets.has_secret(product_id, &input_data.name) {
            return HttpFailResult::as_validation_error("Secret already exists").into();
        }
    }

    let random_value = crate::secret_generator::generate(input_data.length);

    let removed = crate::flows::save_secret(
        &action.app,
        product_id,
        input_data.name,
        random_value,
        input_data.level,
    )
    .await;

    match removed {
        Some(removed) => HttpOutput::as_text(removed.content.to_string()).into_ok_result(false),
        None => HttpOutput::as_text(String::new()).into_ok_result(false),
    }
}
