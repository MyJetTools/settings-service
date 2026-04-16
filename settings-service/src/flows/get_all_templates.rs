use crate::{app_ctx::AppContext, templates_grpc::*};

pub async fn get_all_templates(app: &AppContext) -> Vec<TemplateListItemGrpcModel> {
    let secrets = app.secrets.get_snapshot().await;

    let mut result_templates = app
        .templates
        .get_all(|product_id, item| {
            let has_missing_placeholders = item
                .content
                .has_missing_placeholders(product_id.into(), secrets.as_ref());
            TemplateListItemGrpcModel {
                product_id: product_id.to_string(),
                template_id: item.id.to_string(),
                created: item.created.to_rfc3339(),
                updated: item.last_update.to_rfc3339(),
                last_requests: Default::default(),
                has_missing_placeholders,
            }
        })
        .await;

    let last_template_access_list = app.last_time_access.lock().await;

    for template in result_templates.iter_mut() {
        let last_request =
            last_template_access_list.get(template.product_id.as_str(), &template.template_id);

        template.last_requests = match last_request {
            Some(last_request) => last_request.unix_microseconds,
            None => 0,
        };
    }

    result_templates
}
