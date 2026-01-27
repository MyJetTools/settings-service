use crate::{app_ctx::AppContext, templates_grpc::*};

pub async fn get_all_templates(app: &AppContext) -> Vec<TemplateListItemGrpcModel> {
    let mut result = Vec::new();
    let all_templates = app.templates.get_all().await;

    let secrets = app.secrets.get_snapshot().await;

    for (product_id, items) in all_templates {
        for item in items {
            let last_request = app
                .last_time_access
                .get(product_id.as_str(), &item.id)
                .await;

            let last_request = match last_request {
                Some(last_request) => last_request.unix_microseconds,
                None => 0,
            };

            let has_missing_placeholders = item
                .content
                .has_missing_placeholders(product_id.as_str().into(), secrets.as_ref());

            result.push(TemplateListItemGrpcModel {
                product_id: product_id.to_string(),
                template_id: item.id.to_string(),
                created: item.created.to_rfc3339(),
                updated: item.last_update.to_rfc3339(),
                last_requests: last_request,
                has_missing_placeholders,
            });
        }
    }

    result
}
