use my_http_server::{
    macros::{MyHttpInput, MyHttpObjectStructure},
    FileContent,
};
use my_http_utils::{
    http_input::{HttpInputValue, HttpParseError},
    schema::data_types::{DataTypeProvider, HttpDataType},
};
use serde::{Deserialize, Serialize};

use crate::models::TemplateItem;

/// `#[http_form_data]` file field. `FileContent` has no `DataTypeProvider`/`as_str` impl in
/// my-http-utils 0.1.0 (the derive's client-request-builder codegen still expects every custom
/// struct field to behave like a string wrapper, even though it is never used to build a client
/// request here), so this thin wrapper supplies both.
pub struct SnapshotFile(pub FileContent);

impl DataTypeProvider for SnapshotFile {
    fn get_data_type() -> HttpDataType {
        HttpDataType::as_binary()
    }
}

impl SnapshotFile {
    pub fn as_str(&self) -> &str {
        self.0.file_name.as_str()
    }
}

impl<'s> TryFrom<HttpInputValue<'s>> for SnapshotFile {
    type Error = HttpParseError;
    fn try_from(value: HttpInputValue<'s>) -> Result<Self, Self::Error> {
        Ok(SnapshotFile(value.try_into()?))
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TemplateHttpModel {
    pub product_id: String,
    pub template_id: String,
    pub created: i64,
    pub updated: i64,
    pub last_requests: i64,
    pub has_missing_placeholders: bool,
}

#[derive(MyHttpInput)]
pub struct GetTemplateContentInput {
    #[http_query(description = "Product id")]
    pub product_id: String,
    #[http_query(description = "Template id")]
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TemplateContentHttpModel {
    pub content: String,
}

#[derive(MyHttpInput)]
pub struct SaveTemplateInput {
    #[http_body(description = "Product id")]
    pub product_id: String,
    #[http_body(description = "Template id")]
    pub template_id: String,
    #[http_body(description = "Yaml content")]
    pub yaml: String,
}

#[derive(MyHttpInput)]
pub struct DeleteTemplateInput {
    #[http_body(description = "Product id")]
    pub product_id: String,
    #[http_body(description = "Template id")]
    pub template_id: String,
}

#[derive(MyHttpInput)]
pub struct CompileYamlInput {
    #[http_body(description = "Product id")]
    pub product_id: String,
    #[http_body(description = "Template id")]
    pub template_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct CompiledYamlHttpModel {
    pub yaml: String,
    pub remote_yaml: Option<String>,
    pub local_env_prefixes: Vec<String>,
}

#[derive(MyHttpInput)]
pub struct SnapshotExportInput {
    #[http_query(description = "Product id")]
    pub product_id: String,
    #[http_query(description = "Export templates only (no secrets)")]
    pub templates_only: Option<bool>,
}

#[derive(MyHttpInput)]
pub struct SnapshotImportInput {
    #[http_query(description = "Product id")]
    pub product_id: String,
    #[http_form_data(name = "snapshot", description = "Snapshot file (json)")]
    pub snapshot: SnapshotFile,
}

pub fn template_to_http_model(
    product_id: &str,
    item: &TemplateItem,
    last_requests: i64,
    has_missing_placeholders: bool,
) -> TemplateHttpModel {
    TemplateHttpModel {
        product_id: product_id.to_string(),
        template_id: item.id.clone(),
        created: item.created.unix_microseconds,
        updated: item.last_update.unix_microseconds,
        last_requests,
        has_missing_placeholders,
    }
}
