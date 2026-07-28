use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "POST",
    route: "/api/v1/secrets",
    description: "Save (create or update) a secret",
    summary: "Save secret",
    controller: "V1::Secrets",
    input_data: SaveSecretInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SaveSecretAction {
    app: Arc<AppContext>,
}

impl SaveSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SaveSecretAction,
    input_data: SaveSecretInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product_id: ProductId = input_data.product_id.as_deref().into();

    crate::flows::save_secret(
        &action.app,
        product_id,
        input_data.secret_id,
        input_data.value,
        input_data.remote_value,
        input_data.level as u8,
        input_data.description,
        Some(input_data.visible_for_mcp),
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
