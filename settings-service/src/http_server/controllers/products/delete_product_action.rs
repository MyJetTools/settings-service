use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/products/delete",
    description: "Delete product",
    summary: "Removes the product description and prompt; secrets/templates of that product are preserved",
    controller: "Products",
    input_data: "DeleteProductContract",
    result:[
        {status_code: 202, description: "Ok response"},
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
    input_data: DeleteProductContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::delete_product(&action.app, &input_data.id).await;
    HttpOutput::Empty.into_ok_result(false)
}
