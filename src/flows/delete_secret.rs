use crate::{app_ctx::AppContext, models::ProductId};

pub async fn delete_secret(app: &AppContext, product_id: ProductId<'_>, secret_id: &str) {
    app.secrets.remove(product_id, secret_id).await;

    let secrets_snapshot = app.secrets.get_snapshot().await;
    app.secrets_persistence.save(&secrets_snapshot).await;
}
