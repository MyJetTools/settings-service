use crate::{app_ctx::AppContext, caches::SecretsSnapshot, models::*, secrets_grpc::*};

pub async fn to_secret_grpc_model(
    app: &AppContext,
    product_id: ProductId<'_>,
    secrets: &SecretsSnapshot,
    secret_item: &SecretItem,
) -> SecretGrpcModel {
    let used_by_secrets = match product_id {
        ProductId::Shared => secrets.get_count(product_id, |itm| {
            itm.content.has_the_secret_inside(&secret_item.id)
        }),
        ProductId::Id(product_id) => {
            let by_product = secrets.get_count(ProductId::Id(product_id), |itm| {
                itm.content.has_the_secret_inside(&secret_item.id)
            });

            let shared = secrets.get_count(ProductId::Shared, |itm| {
                itm.content.has_the_secret_inside(&secret_item.id)
            });

            by_product + shared
        }
    };

    let used_by_templates = app
        .templates
        .get_count_from_all(|itm| itm.content.has_the_secret_inside(&secret_item.id))
        .await;

    SecretGrpcModel {
        secret_id: secret_item.id.to_string(),
        level: secret_item.level as i32,
        created: secret_item.created.to_rfc3339(),
        updated: secret_item.updated.to_rfc3339(),
        used_by_secrets: used_by_secrets as i32,
        used_by_templates: used_by_templates as i32,
        product_id: match product_id {
            ProductId::Shared => None,
            ProductId::Id(product_id) => Some(product_id.to_string()),
        },
    }
}

impl Into<SecretValueGrpcModel> for &SecretItem {
    fn into(self) -> SecretValueGrpcModel {
        SecretValueGrpcModel {
            level: self.level as i32,
            value: self.content.to_string(),
        }
    }
}
