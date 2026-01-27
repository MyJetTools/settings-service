use crate::app_ctx::AppContext;

pub async fn compile_yaml(app: &AppContext, product_id: &str, template_id: &str) -> Option<String> {
    let secrets_snapshot = app.secrets.get_snapshot().await;

    let content = app
        .templates
        .get_by_id(product_id, template_id, |itm| itm.content.clone())
        .await?;

    let result =
        crate::scripts::populate_secrets(app, product_id.into(), &content, &secrets_snapshot, 0);

    Some(result.into_string())
}
