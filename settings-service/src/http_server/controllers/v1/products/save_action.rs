use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/products",
    description: "Save (create or update) a product",
    summary: "Save product",
    controller: "V1::Products",
    input_data: SaveProductInput,
    result: [
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SaveProductAction {
    app: Arc<AppContext>,
}

impl SaveProductAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SaveProductAction,
    input_data: SaveProductInput,
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
