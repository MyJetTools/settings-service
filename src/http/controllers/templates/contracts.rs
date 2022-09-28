use std::sync::Arc;

use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

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
    pub async fn new(app: &AppContext, items: Vec<Arc<TemplateMyNoSqlEntity>>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        let time_snapshot = app.last_request.get_snapshot().await;

        for item in items {
            let last_time = if let Some(sub_itmes) = time_snapshot.get(&item.partition_key) {
                if let Some(value) = sub_itmes.get(&item.row_key) {
                    value.unix_microseconds / 1000
                } else {
                    0
                }
            } else {
                0
            };

            data.push(SettingTemplateModel::new(&item, last_time));
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
    #[serde(rename = "lastRequest")]
    last_request: i64,
}

impl SettingTemplateModel {
    pub fn new(itm: &TemplateMyNoSqlEntity, last_request: i64) -> Self {
        Self {
            env: itm.partition_key.clone(),
            name: itm.row_key.clone(),
            created: itm.create_date.clone(),
            updated: itm.last_update_date.clone(),
            last_request,
        }
    }
}
