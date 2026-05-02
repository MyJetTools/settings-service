use std::sync::Arc;

use mcp_server_middleware::McpMiddleware;

use crate::app_ctx::AppContext;

const MCP_PATH: &str = "/mcp";
const MCP_NAME: &str = "SettingsService";
const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const MCP_INSTRUCTIONS: &str = "SettingsService MCP server. Manages YAML configuration templates and the secrets they reference. Workflow when wiring up an app: (1) call `list_secrets` to discover which secrets already exist for a product (with their human-readable descriptions); (2) call `get_secret_info` to read a specific secret's value, description, and whether it has a remote-datacenter variant; (3) call `compile_template_yaml` to read the rendered YAML for a known product+template — missing secret references appear inline as `/*Secret <id> is not found*/` and are also returned in `missing_keys`; (4) call `upsert_template` to create or overwrite a template's YAML body. Use \"Shared\" as the product_id for shared templates and shared secrets.";

pub async fn build_mcp_middleware(app: &Arc<AppContext>) -> McpMiddleware {
    let mut middleware = McpMiddleware::new(MCP_PATH, MCP_NAME, MCP_VERSION, MCP_INSTRUCTIONS);

    middleware
        .register_tool_call(Arc::new(super::CompileTemplateYamlHandler::new(app.clone())))
        .await;

    middleware
        .register_tool_call(Arc::new(super::UpsertTemplateHandler::new(app.clone())))
        .await;

    middleware
        .register_tool_call(Arc::new(super::GetSecretInfoHandler::new(app.clone())))
        .await;

    middleware
        .register_tool_call(Arc::new(super::ListSecretsHandler::new(app.clone())))
        .await;

    middleware
}
