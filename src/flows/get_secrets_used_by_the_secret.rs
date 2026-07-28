use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::*};

pub async fn get_secrets_used_by_the_secret(
    app: &AppContext,
    product_id: ProductId<'_>,
    secret_id: &str,
) -> Vec<SecretUsageGrpcModel> {
    let secrets_snapshot = app.secrets.get_snapshot().await;

    let product_id = match product_id {
        ProductId::Shared => {
            let result = secrets_snapshot.find_all_into_vec(|itm| {
                if itm.content.has_the_secret_inside(secret_id) {
                    Some(SecretUsageGrpcModel {
                        product_id: String::new(),
                        id: itm.id.to_string(),
                        value: itm.content.to_string(),
                    })
                } else {
                    None
                }
            });

            return result;
        }
        ProductId::Id(product_id) => product_id,
    };

    let mut result = secrets_snapshot.find_into_vec_by_product(product_id.into(), |itm| {
        if itm.content.has_the_secret_inside(secret_id) {
            Some(SecretUsageGrpcModel {
                product_id: product_id.to_string(),
                id: itm.id.to_string(),
                value: itm.content.to_string(),
            })
        } else {
            None
        }
    });

    let shared = secrets_snapshot.find_into_vec_by_product(ProductId::Shared, |itm| {
        if itm.content.has_the_secret_inside(secret_id) {
            Some(SecretUsageGrpcModel {
                product_id: String::new(),
                id: itm.id.to_string(),
                value: itm.content.to_string(),
            })
        } else {
            None
        }
    });

    result.extend(shared);

    result
}
