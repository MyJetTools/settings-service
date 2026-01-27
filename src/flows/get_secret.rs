use crate::{app_ctx::AppContext, models::ProductId, secrets_grpc::*};

pub async fn get_secret(
    app: &AppContext,
    product_id: ProductId<'_>,
    secret_id: &str,
) -> SecretValueGrpcModel {
    let secrets = app.secrets.get_snapshot().await;
    let result = secrets.get_by_id(product_id, secret_id);

    if let Some(item) = result {
        return item.into();
    }
    SecretValueGrpcModel::default()
}
