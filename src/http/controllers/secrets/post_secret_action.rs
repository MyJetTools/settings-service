use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/secrets/post",
    description: "Save secret",
    summary: "Saves secret",
    controller: "Secrets",
    input_data: "PostSecretContract",
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct PostSecretAction {
    app: Arc<AppContext>,
}

impl PostSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostSecretAction,
    input_data: PostSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    action
        .app
        .key_value_repository
        .set_secret(input_data.name.as_str(), input_data.secret.as_str())
        .await;

    HttpOutput::Empty.into_ok_result(false)
}
