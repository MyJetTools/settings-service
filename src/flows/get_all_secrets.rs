use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::*};

pub async fn get_all_secrets(app: &AppContext, product_id: ProductId<'_>) -> Vec<SecretGrpcModel> {
    let secrets = app.secrets.get_snapshot().await;

    let slice = secrets.get_slice(product_id);

    let mut result = Vec::with_capacity(slice.len());

    for item in slice {
        let item = crate::mappers::to_secret_grpc_model(app, product_id, &secrets, item).await;
        result.push(item);
    }

    result
}
