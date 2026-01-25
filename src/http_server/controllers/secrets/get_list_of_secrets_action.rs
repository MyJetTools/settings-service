use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpInput},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/secrets/getall",
    description: "Get list of secrets",
    summary: "Returns list of secrets",
    controller: "Secrets",
    input_data: GetListOfSecretsHttpInput,
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
    input_data: GetListOfSecretsHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let secrets = crate::scripts::secrets::get_all(&action.app, input_data.env.as_deref())
        .await
        .unwrap_or(vec![]);

    let result = ListOfSecretsContract::new(&action.app, input_data.env.as_deref(), secrets).await;
    HttpOutput::as_json(result).into_ok_result(false)
}

#[derive(Debug, MyHttpInput)]
pub struct GetListOfSecretsHttpInput {
    #[http_query(description: "Environment")]
    pub env: Option<String>,
}
