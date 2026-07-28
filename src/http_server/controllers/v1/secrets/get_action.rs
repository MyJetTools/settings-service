use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "GET",
    route: "/api/v1/secrets/get",
    description: "Get a single secret with its value",
    summary: "Get secret",
    controller: "V1::Secrets",
    input_data: GetSecretInput,
    result: [
        {status_code: 200, description: "Ok response", model: "SecretValueHttpModel"},
        {status_code: 404, description: "Secret not found"},
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
    input_data: GetSecretInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let snapshot = action.app.secrets.get_snapshot().await;
    let product_id: ProductId = input_data.product_id.as_deref().into();

    let item = snapshot.get_by_id(product_id, &input_data.secret_id);
    let Some(item) = item else {
        return Err(HttpFailResult::as_not_found(
            "Secret not found".to_string(),
            false,
        ));
    };

    let model = SecretValueHttpModel {
        value: item.content.to_string(),
        level: item.level as i32,
        remote_value: item.remote_value.as_ref().map(|c| c.to_string()),
        description: item.description.clone(),
        visible_for_mcp: item.visible_for_mcp,
    };

    HttpOutput::as_json(model).into_ok_result(false)
}
