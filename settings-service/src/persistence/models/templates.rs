use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

use crate::models::*;
#[derive(Default, Serialize, Deserialize)]
pub struct TemplateFileData {
    pub content: String,
    pub created: i64,
    pub updated: i64,
}

impl Into<TemplateItem> for (String, TemplateFileData) {
    fn into(self) -> TemplateItem {
        TemplateItem {
            id: self.0,
            content: Content::from_base_64(self.1.content.as_str()),
            created: DateTimeAsMicroseconds::new(self.1.created),
            last_update: DateTimeAsMicroseconds::new(self.1.updated),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TemplatesFileModel {
    pub templates: BTreeMap<String, BTreeMap<String, TemplateFileData>>,
}

impl TemplatesFileModel {
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_slice(src: &[u8]) -> Self {
        serde_json::from_slice(src).unwrap()
    }
}
