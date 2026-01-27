use std::collections::BTreeMap;

use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct TemplatesFileModel {
    pub items: BTreeMap<String, BTreeMap<String, String>>,
}

impl TemplatesFileModel {
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_slice(src: &[u8]) -> Self {
        serde_json::from_slice(src).unwrap()
    }
}
