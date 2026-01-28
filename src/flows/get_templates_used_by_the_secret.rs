use crate::{app_ctx::AppContext, secrets_grpc::TemplateUsageGrpcModel};

pub async fn get_templates_used_by_the_secret(
    app: &AppContext,
    product_id: &str,
    secret_id: &str,
) -> Vec<TemplateUsageGrpcModel> {
    app.templates
        .find_into_vec(|template| {
            if template.content.has_the_secret_inside(secret_id) {
                let item = TemplateUsageGrpcModel {
                    product: product_id.to_string(),
                    template_id: template.id.to_string(),
                    template_content: template.content.to_string(),
                };

                Some(item)
            } else {
                None
            }
        })
        .await
}
