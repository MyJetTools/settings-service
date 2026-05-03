use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/products/get",
    description: "Get product",
    summary: "Returns a single product",
    controller: "Products",
    input_data: "GetProductContract",
    result:[
        {status_code: 200, description: "Ok response", model: "ProductHttpModel"},
    ]
)]
pub struct GetProductAction {
    app: Arc<AppContext>,
}

impl GetProductAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetProductAction,
    input_data: GetProductContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let snapshot = action.app.products.get_snapshot().await;

    let model = if let Some(product) = snapshot.get(input_data.id.as_str()) {
        ProductHttpModel {
            id: product.id.clone(),
            description: Some(product.description.clone()),
            prompt: Some(product.prompt.clone()),
            templates_amount: 0,
            secrets_amount: 0,
            has_metadata: true,
        }
    } else {
        return Err(HttpFailResult::as_not_found(
            "Product not found".to_string(),
            false,
        ));
    };

    HttpOutput::as_json(model).into_ok_result(false)
}
