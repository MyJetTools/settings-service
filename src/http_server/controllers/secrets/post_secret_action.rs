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
    mut input_data: PostSecretContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product = std::mem::take(&mut input_data.product);
    let name = std::mem::take(&mut input_data.name);
    crate::flows::save_secret(
        &action.app,
        product.as_deref().into(),
        name,
        input_data.secret,
        input_data.level,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
