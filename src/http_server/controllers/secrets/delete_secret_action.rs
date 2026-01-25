use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/delete",
    description: "Delete secret",
    summary: "Deletes secret",
    controller: "Secrets",
    input_data: "DeleteSecretInputContract",
    result:[
        {status_code: 202, description: "Ok response"},
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
    input_data: DeleteSecretInputContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::scripts::secrets::delete(&action.app, input_data.env.as_deref(), &input_data.name).await;
    HttpOutput::Empty.into_ok_result(false)
}
