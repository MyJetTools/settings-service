use crate::app_ctx::AppContext;

pub async fn get_populated_template(app: &AppContext, env: &str, name: &str) -> Option<String> {
    println!("Getting populated template {}/{}", env, name);
    let template = super::get(app, env, name).await?;
    crate::scripts::populate_with_secrets(app, template.yaml_template.as_str())
        .await
        .into()
}
