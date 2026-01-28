use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::*};

pub async fn get_all_secrets(
    app: &AppContext,
    product_id: &str,
    include_shared: bool,
) -> Vec<SecretGrpcModel> {
    let secrets = app.secrets.get_snapshot().await;

    let mut result = Vec::new();

    if let Some(by_product) = secrets.by_product.get(product_id) {
        for item in by_product.iter() {
            let item = crate::mappers::to_secret_grpc_model(
                app,
                ProductId::Id(product_id),
                &secrets,
                item,
            )
            .await;
            result.push(item);
        }
    }

    if include_shared {
        for itm in secrets.shared.iter() {
            let item =
                crate::mappers::to_secret_grpc_model(app, ProductId::Shared, &secrets, itm).await;
            result.push(item);
        }
    }

    result
}
