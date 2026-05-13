use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/v1/templates",
    description: "List all templates",
    summary: "List all templates",
    controller: "V1::Templates",
    result: [
        {status_code: 200, description: "Array of TemplateHttpModel"},
    ]
)]
pub struct ListTemplatesAction {
    app: Arc<AppContext>,
}

impl ListTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ListTemplatesAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let secrets = action.app.secrets.get_snapshot().await;

    let mut result = action
        .app
        .templates
        .get_all(|product_id, item| {
            let has_missing_placeholders = item
                .content
                .has_missing_placeholders(product_id.into(), secrets.as_ref());
            template_to_http_model(product_id, item, 0, has_missing_placeholders)
        })
        .await;

    let last_access = action.app.last_time_access.lock().await;
    for template in result.iter_mut() {
        template.last_requests = last_access
            .get(template.product_id.as_str(), &template.template_id)
            .map(|d| d.unix_microseconds)
            .unwrap_or(0);
    }

    HttpOutput::as_json(result).into_ok_result(false)
}
