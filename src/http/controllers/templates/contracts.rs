use std::sync::Arc;

use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::my_no_sql::TemplateMyNoSqlEntity;

#[derive(MyHttpInput)]
pub struct DeleteTemplateContract {
    #[http_form(description = "Environment")]
    pub env: String,
    #[http_form(description = "Service name")]
    pub name: String,
}

#[derive(MyHttpInput)]
pub struct PostTemplateContract {
    #[http_form(description = "Environment")]
    pub env: String,
    #[http_form(description = "Service name")]
    pub name: String,
    #[http_form(description = "Yaml template")]
    pub yaml: String,
}

#[derive(MyHttpInput)]
pub struct GetTemplateContract {
    #[http_form(description = "Environment")]
    pub env: String,
    #[http_form(description = "Service name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfTemplatesContract {
    data: Vec<SettingTemplateModel>,
}

impl ListOfTemplatesContract {
    pub fn new(items: Vec<Arc<TemplateMyNoSqlEntity>>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for item in items {
            data.push(SettingTemplateModel::new(&item));
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SettingTemplateModel {
    env: String,
    name: String,
    created: String,
    updated: String,
}

impl SettingTemplateModel {
    pub fn new(itm: &TemplateMyNoSqlEntity) -> Self {
        Self {
            env: itm.partition_key.clone(),
            name: itm.row_key.clone(),
            created: itm.create_date.clone(),
            updated: itm.last_update_date.clone(),
        }
    }
}
