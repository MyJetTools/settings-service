use crate::app_ctx::AppContext;

pub async fn delete_product(app: &AppContext, id: &str) {
    let snapshot = app.products.delete(id).await;
    app.products_persistence.save_all(&snapshot.as_vec()).await;
}
