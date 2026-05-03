use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/products/getall",
    description: "Get list of products",
    summary: "Returns the union of explicit products and implicit products discovered from secrets/templates",
    controller: "Products",
    input_data: "GetProductsListContract",
    result:[
        {status_code: 200, description: "Ok response", model: "ListOfProductsContract"},
    ]
)]
pub struct GetProductsAction {
    app: Arc<AppContext>,
}

impl GetProductsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetProductsAction,
    _input_data: GetProductsListContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let products = crate::flows::get_all_products(&action.app).await;

    let result = ListOfProductsContract {
        data: products.into_iter().map(Into::into).collect(),
    };

    HttpOutput::as_json(result).into_ok_result(false)
}
