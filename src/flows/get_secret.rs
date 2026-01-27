use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::*};

pub async fn get_secret(
    app: &AppContext,
    product_id: ProductId<'_>,
    secret_id: &str,
) -> SecretGrpcModel {
    let secrets = app.secrets.get_snapshot().await;
    let result = secrets.get_by_id(product_id, secret_id);
    match result {
        Some(item) => {
            crate::mappers::to_secret_grpc_model(&app, product_id, &secrets, item.as_ref()).await
        }
        None => SecretGrpcModel::default(),
    }
}
