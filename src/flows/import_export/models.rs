use serde::*;

use crate::{models::*, persistence::TemplatePersistenceItem};

#[derive(Serialize, Deserialize)]
pub struct SnapshotExportModel {
    pub templates: Vec<TemplateExportModel>,
    pub secrets: Vec<SecretExportModel>,
}

impl SnapshotExportModel {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TemplateExportModel {
    pub id: String,
    pub content: String,
}

impl TemplateExportModel {
    pub fn from_cache_item(src: &TemplateItem) -> Self {
        Self {
            id: src.id.to_string(),
            content: src.content.to_base_64(),
        }
    }
}

impl TemplatePersistenceItem for TemplateExportModel {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SecretExportModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<u64>,
    pub id: String,
    pub value: String,
    pub level: u8,
}
