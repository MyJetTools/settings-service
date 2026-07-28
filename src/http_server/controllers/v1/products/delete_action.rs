use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/products/delete",
    description: "Delete a product",
    summary: "Delete product",
    controller: "V1::Products",
    input_data: DeleteProductInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct DeleteProductAction {
    app: Arc<AppContext>,
}

impl DeleteProductAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteProductAction,
    input_data: DeleteProductInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::delete_product(&action.app, &input_data.product_id).await;

    HttpOutput::Empty.into_ok_result(false)
}
