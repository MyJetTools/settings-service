use crate::app_ctx::AppContext;

pub async fn get_populated_template(app: &AppContext, evn: &str, name: &str) -> Option<String> {
    let template = super::get(app, evn, name).await?;
    crate::scripts::populate_with_secrets(app, template.yaml_template.as_str())
        .await
        .into()
}
