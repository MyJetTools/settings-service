use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::TemplateUsageGrpcModel};

pub async fn get_templates_used_by_the_secret(
    app: &AppContext,
    product_id: ProductId<'_>,
    secret_id: &str,
) -> Vec<TemplateUsageGrpcModel> {
    match product_id {
        ProductId::Shared => {
            let result = app
                .templates
                .find_into_vec(|product_id, template| {
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
                .await;
            result
        }
        ProductId::Id(product_id) => {
            let result = app
                .templates
                .find_into_vec_by_product(product_id, |template| {
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
                .await;
            result
        }
    }
}
