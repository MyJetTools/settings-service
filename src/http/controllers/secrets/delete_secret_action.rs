use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/secrets/delete",
    description: "Delete secret",
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
    action
        .app
        .key_value_repository
        .delete_secret(input_data.name.as_str())
        .await;

    HttpOutput::Empty.into_ok_result(false)
}
