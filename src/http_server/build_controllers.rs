use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app_ctx::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> Arc<ControllersMiddleware> {
    let mut result = ControllersMiddleware::new(None, None);

    // v1::envs
    result.register_get_action(Arc::new(
        super::controllers::v1::envs::GetEnvInfoAction::new(app.clone()),
    ));

    // v1::templates
    result.register_get_action(Arc::new(
        super::controllers::v1::templates::ListTemplatesAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::templates::GetTemplateContentAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::templates::SaveTemplateAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::templates::DeleteTemplateAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::templates::CompileYamlAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::templates::SnapshotExportAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::templates::SnapshotImportAction::new(app.clone()),
    ));

    // v1::secrets
    result.register_get_action(Arc::new(
        super::controllers::v1::secrets::ListSecretsAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::secrets::GetSecretAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::secrets::SaveSecretAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::secrets::DeleteSecretAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::secrets::UsageByTemplatesAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::secrets::UsageBySecretsAction::new(app.clone()),
    ));

    // v1::products
    result.register_get_action(Arc::new(
        super::controllers::v1::products::ListProductsAction::new(app.clone()),
    ));
    result.register_get_action(Arc::new(
        super::controllers::v1::products::GetProductAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::products::SaveProductAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::v1::products::DeleteProductAction::new(app.clone()),
    ));

    Arc::new(result)
}
