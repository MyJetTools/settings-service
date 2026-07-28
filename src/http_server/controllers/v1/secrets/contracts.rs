use my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretHttpModel {
    pub product_id: Option<String>,
    pub secret_id: String,
    pub level: i32,
    pub created: i64,
    pub updated: i64,
    pub used_by_templates: i32,
    pub used_by_secrets: i32,
    pub description: Option<String>,
    pub visible_for_mcp: bool,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretValueHttpModel {
    pub value: String,
    pub level: i32,
    pub remote_value: Option<String>,
    pub description: Option<String>,
    pub visible_for_mcp: bool,
}

#[derive(MyHttpInput)]
pub struct ListSecretsInput {
    #[http_query(description = "Product id (empty for shared)")]
    pub product_id: Option<String>,
    #[http_query(description = "Include shared secrets in result (defaults to true)")]
    pub include_shared: Option<bool>,
}

#[derive(MyHttpInput)]
pub struct GetSecretInput {
    #[http_query(description = "Product id (empty for shared)")]
    pub product_id: Option<String>,
    #[http_query(description = "Secret id")]
    pub secret_id: String,
}

#[derive(MyHttpInput)]
pub struct SaveSecretInput {
    #[http_body(description = "Product id (empty for shared)")]
    pub product_id: Option<String>,
    #[http_body(description = "Secret id")]
    pub secret_id: String,
    #[http_body(description = "Plain value")]
    pub value: String,
    #[http_body(description = "Visibility level")]
    pub level: i32,
    #[http_body(description = "Remote value override")]
    pub remote_value: Option<String>,
    #[http_body(description = "Optional description")]
    pub description: Option<String>,
    #[http_body(description = "Visible to MCP/AI agents")]
    pub visible_for_mcp: bool,
}

#[derive(MyHttpInput)]
pub struct DeleteSecretInput {
    #[http_body(description = "Product id (empty for shared)")]
    pub product_id: Option<String>,
    #[http_body(description = "Secret id")]
    pub secret_id: String,
}

#[derive(MyHttpInput)]
pub struct SecretUsageInput {
    #[http_query(description = "Product id (empty for shared)")]
    pub product_id: Option<String>,
    #[http_query(description = "Secret id")]
    pub secret_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretUsageBySecretHttpModel {
    pub product_id: Option<String>,
    pub secret_id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TemplateUsageHttpModel {
    pub product_id: String,
    pub template_id: String,
    pub yaml: String,
}
