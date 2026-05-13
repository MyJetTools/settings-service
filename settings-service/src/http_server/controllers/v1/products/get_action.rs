use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/v1/products/get",
    description: "Get a single product by id",
    summary: "Get product",
    controller: "V1::Products",
    input_data: GetProductInput,
    result: [
        {status_code: 200, description: "Ok response", model: "ProductHttpModel"},
        {status_code: 404, description: "Product not found"},
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
    input_data: GetProductInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let products = crate::flows::get_all_products(&action.app).await;

    let item = products.into_iter().find(|p| p.id == input_data.product_id);

    let Some(item) = item else {
        return Err(HttpFailResult::as_not_found(
            "Product not found".to_string(),
            false,
        ));
    };

    let model = ProductHttpModel {
        id: item.id,
        description: item.description,
        prompt: item.prompt,
        templates_amount: item.templates_count,
        secrets_amount: item.secrets_count,
        has_metadata: item.has_metadata,
    };

    HttpOutput::as_json(model).into_ok_result(false)
}
