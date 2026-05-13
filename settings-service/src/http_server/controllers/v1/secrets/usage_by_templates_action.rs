use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "GET",
    route: "/api/v1/secrets/usage/by-templates",
    description: "List templates that reference the secret",
    summary: "Secret usage by templates",
    controller: "V1::Secrets",
    input_data: SecretUsageInput,
    result: [
        {status_code: 200, description: "Array of TemplateUsageHttpModel"},
    ]
)]
pub struct UsageByTemplatesAction {
    app: Arc<AppContext>,
}

impl UsageByTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &UsageByTemplatesAction,
    input_data: SecretUsageInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let product_id: ProductId = input_data.product_id.as_deref().into();

    let usage = crate::flows::get_templates_used_by_the_secret(
        &action.app,
        product_id,
        &input_data.secret_id,
    )
    .await;

    let result: Vec<TemplateUsageHttpModel> = usage
        .into_iter()
        .map(|itm| TemplateUsageHttpModel {
            product_id: itm.product,
            template_id: itm.template_id,
            yaml: itm.template_content,
        })
        .collect();

    HttpOutput::as_json(result).into_ok_result(false)
}
