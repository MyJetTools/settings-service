use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/v1/products",
    description: "List all products (with counts and metadata)",
    summary: "List products",
    controller: "V1::Products",
    result: [
        {status_code: 200, description: "Array of ProductHttpModel"},
    ]
)]
pub struct ListProductsAction {
    app: Arc<AppContext>,
}

impl ListProductsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ListProductsAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let products = crate::flows::get_all_products(&action.app).await;

    let result: Vec<ProductHttpModel> = products
        .into_iter()
        .map(|item| ProductHttpModel {
            id: item.id,
            description: item.description,
            prompt: item.prompt,
            templates_amount: item.templates_count,
            secrets_amount: item.secrets_count,
            has_metadata: item.has_metadata,
        })
        .collect();

    HttpOutput::as_json(result).into_ok_result(false)
}
