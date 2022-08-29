use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app_ctx::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> Arc<ControllersMiddleware> {
    let mut result = ControllersMiddleware::new();

    result.register_post_action(Arc::new(super::controllers::secrets::GetSecretAction::new(
        app.clone(),
    )));

    result.register_post_action(Arc::new(
        super::controllers::secrets::GetSecretsAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        super::controllers::templates::GetTemplatesAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        super::controllers::templates::GetTemplateAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(super::controllers::home::IndexAction::new(
        app.clone(),
    )));

    Arc::new(result)
}
