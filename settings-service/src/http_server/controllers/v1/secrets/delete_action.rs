use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "POST",
    route: "/api/v1/secrets/delete",
    description: "Delete a secret",
    summary: "Delete secret",
    controller: "V1::Secrets",
    input_data: DeleteSecretInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct DeleteSecretAction {
    app: Arc<AppContext>,
}

impl DeleteSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteSecretAction,
    input_data: DeleteSecretInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product_id: ProductId = input_data.product_id.as_deref().into();

    crate::flows::delete_secret(&action.app, product_id, &input_data.secret_id).await;

    HttpOutput::Empty.into_ok_result(false)
}
