use crate::app_ctx::AppContext;

pub async fn init(app: &AppContext) {
    let templates = app.templates_persistence.get_file_content().await;

    for (product_id, templates) in templates.items {
        let iterator = templates.into_iter().map(|itm| itm.into());
        app.templates.insert(product_id.as_str(), iterator).await;
    }

    let secrets = app.secrets_persistence.get_all().await;
    app.secrets.init(secrets).await;

    app.app_states.set_initialized();
}
