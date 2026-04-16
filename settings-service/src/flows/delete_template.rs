use crate::app_ctx::AppContext;

pub async fn delete_template(app: &AppContext, product_id: &str, template_id: &str) {
    app.templates_persistence
        .delete(product_id, template_id)
        .await;

    app.templates.remove(product_id, template_id).await;
}
