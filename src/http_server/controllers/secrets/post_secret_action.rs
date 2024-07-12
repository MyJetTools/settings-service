use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
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
    crate::operations::update_secret(&action.app, input_data.name.to_string(), input_data.into())
        .await;

    HttpOutput::Empty.into_ok_result(false)
}
