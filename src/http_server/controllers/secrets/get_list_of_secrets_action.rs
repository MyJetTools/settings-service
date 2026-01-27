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
    let secrets = action.app.secrets.get_snapshot().await;
    let secrets = secrets
        .get_all_by_product_id(input_data.product.as_deref().into())
        .await;

    let result =
        ListOfSecretsContract::new(&action.app, input_data.product.as_deref().into(), secrets)
            .await;
    HttpOutput::as_json(result).into_ok_result(false)
}

#[derive(Debug, MyHttpInput)]
pub struct GetListOfSecretsHttpInput {
    #[http_query(description: "Product")]
    pub product: Option<String>,
}
