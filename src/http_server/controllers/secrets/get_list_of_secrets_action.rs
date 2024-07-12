use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/getall",
    description: "Get list of secrets",
    summary: "Returns list of secrets",
    controller: "Secrets",

    result:[
        {status_code: 200, description: "Ok response", model: "ListOfSecretsContract"},
    ]
)]
pub struct GetSecretsAction {
    app: Arc<AppContext>,
}

impl GetSecretsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetSecretsAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let secrets = crate::operations::get_all_secrets(&action.app)
        .await
        .unwrap_or(vec![]);

    let result = ListOfSecretsContract::new(&action.app, secrets).await;
    HttpOutput::as_json(result).into_ok_result(false)
}
