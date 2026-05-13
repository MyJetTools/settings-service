use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "POST",
    route: "/api/v1/templates/yaml",
    description: "Compile template into final yaml with secrets resolved (both local and remote)",
    summary: "Compile yaml",
    controller: "V1::Templates",
    input_data: CompileYamlInput,
    result: [
        {status_code: 200, description: "Ok response", model: "CompiledYamlHttpModel"},
        {status_code: 404, description: "Template not found"},
    ]
)]
pub struct CompileYamlAction {
    app: Arc<AppContext>,
}

impl CompileYamlAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &CompileYamlAction,
    input_data: CompileYamlInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let compiled =
        crate::flows::compile_yaml(&action.app, &input_data.product_id, &input_data.template_id)
            .await;

    let Some(compiled) = compiled else {
        return Err(HttpFailResult::as_not_found(
            "Template not found".to_string(),
            false,
        ));
    };

    let local_env_prefixes = action
        .app
        .settings
        .local_env_prefixes
        .clone()
        .unwrap_or_default();

    let remote_yaml = if compiled.remote == compiled.local {
        None
    } else {
        Some(compiled.remote)
    };

    HttpOutput::as_json(CompiledYamlHttpModel {
        yaml: compiled.local,
        remote_yaml,
        local_env_prefixes,
    })
    .into_ok_result(false)
}
