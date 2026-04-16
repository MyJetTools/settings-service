use crate::app_ctx::AppContext;

pub struct CompiledYaml {
    pub local: String,
    pub remote: String,
}

pub async fn compile_yaml(
    app: &AppContext,
    product_id: &str,
    template_id: &str,
) -> Option<CompiledYaml> {
    let secrets_snapshot = app.secrets.get_snapshot().await;

    let content = app
        .templates
        .get_by_id(product_id, template_id, |itm| itm.content.clone())
        .await?;

    let local = crate::scripts::populate_secrets(
        app,
        product_id.into(),
        &content,
        &secrets_snapshot,
        0,
        false,
    );

    let remote = crate::scripts::populate_secrets(
        app,
        product_id.into(),
        &content,
        &secrets_snapshot,
        0,
        true,
    );

    Some(CompiledYaml {
        local: local.into_string(),
        remote: remote.into_string(),
    })
}
