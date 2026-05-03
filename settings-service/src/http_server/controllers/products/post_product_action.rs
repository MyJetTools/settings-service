use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/products/post",
    description: "Save product",
    summary: "Saves product description and prompt",
    controller: "Products",
    input_data: "PostProductContract",
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct PostProductAction {
    app: Arc<AppContext>,
}

impl PostProductAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostProductAction,
    input_data: PostProductContract,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::save_product(
        &action.app,
        input_data.id,
        input_data.description,
        input_data.prompt,
    )
    .await;

    HttpOutput::Empty.into_ok_result(false)
}
