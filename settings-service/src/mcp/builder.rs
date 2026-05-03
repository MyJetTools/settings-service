use std::sync::Arc;

use mcp_server_middleware::McpMiddleware;

use crate::app_ctx::AppContext;

const MCP_PATH: &str = "/mcp";
const MCP_NAME: &str = "SettingsService";
const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn build_mcp_middleware(app: &Arc<AppContext>) -> McpMiddleware {
    let mut middleware = McpMiddleware::new(
        MCP_PATH,
        MCP_NAME,
        MCP_VERSION,
        super::instructions::MCP_INSTRUCTIONS,
    );

    middleware
        .register_tool_call(Arc::new(super::ListProductsHandler::new(app.clone())))
        .await;

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
