use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{app_ctx::AppContext, models::ProductId};

#[http_route(
    method: "GET",
    route: "/api/v1/secrets",
    description: "List secrets for product (and shared if requested)",
    summary: "List secrets",
    controller: "V1::Secrets",
    input_data: ListSecretsInput,
    result: [
        {status_code: 200, description: "Array of SecretHttpModel"},
    ]
)]
pub struct ListSecretsAction {
    app: Arc<AppContext>,
}

impl ListSecretsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ListSecretsAction,
    input_data: ListSecretsInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let include_shared = input_data.include_shared.unwrap_or(true);
    let snapshot = action.app.secrets.get_snapshot().await;

    let mut result: Vec<SecretHttpModel> = Vec::new();

    if let Some(product_id) = input_data.product_id.as_deref() {
        if let Some(items) = snapshot.by_product.get(product_id) {
            for item in items.iter() {
                let used_by_secrets =
                    snapshot.get_usage(ProductId::Id(product_id), &item.id) as i32;
                let used_by_templates = action
                    .app
                    .templates
                    .get_count_from_all(|t| t.content.has_the_secret_inside(&item.id))
                    .await as i32;
                result.push(SecretHttpModel {
                    product_id: Some(product_id.to_string()),
                    secret_id: item.id.clone(),
                    level: item.level as i32,
                    created: item.created.unix_microseconds,
                    updated: item.updated.unix_microseconds,
                    used_by_templates,
                    used_by_secrets,
                    description: item.description.clone(),
                    visible_for_mcp: item.visible_for_mcp,
                });
            }
        }
    }

    if include_shared {
        for item in snapshot.shared.iter() {
            let used_by_secrets = snapshot.get_usage(ProductId::Shared, &item.id) as i32;
            let used_by_templates = action
                .app
                .templates
                .get_count_from_all(|t| t.content.has_the_secret_inside(&item.id))
                .await as i32;
            result.push(SecretHttpModel {
                product_id: None,
                secret_id: item.id.clone(),
                level: item.level as i32,
                created: item.created.unix_microseconds,
                updated: item.updated.unix_microseconds,
                used_by_templates,
                used_by_secrets,
                description: item.description.clone(),
                visible_for_mcp: item.visible_for_mcp,
            });
        }
    }

    HttpOutput::as_json(result).into_ok_result(false)
}
