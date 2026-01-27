use std::{collections::BTreeMap, sync::Arc};

use my_http_server::macros::*;
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::TemplateItem};

#[derive(MyHttpInput)]
pub struct DeleteTemplateContract {
    #[http_body(description = "Product")]
    pub product: String,
    #[http_body(description = "Service name")]
    pub name: String,
}

#[derive(MyHttpInput)]
pub struct PostTemplateContract {
    #[http_body(description = "Product")]
    pub product: String,
    #[http_body(description = "Service name")]
    pub name: String,
    #[http_body(description = "Yaml template")]
    pub yaml: String,
}

#[derive(MyHttpInput)]
pub struct GetTemplateContract {
    #[http_body(description = "Environment")]
    pub product: String,
    #[http_body(description = "Service name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfTemplatesContract {
    data: Vec<SettingTemplateModel>,
}

impl ListOfTemplatesContract {
    pub async fn new(app: &AppContext, items: BTreeMap<String, Vec<Arc<TemplateItem>>>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for (product_id, item) in items {
            for item in item {
                let last_time = app
                    .last_time_access
                    .get(product_id.as_str(), &item.id)
                    .await;

                let last_time = match last_time {
                    Some(last_time) => last_time.unix_microseconds,
                    None => 0,
                };

                data.push(SettingTemplateModel::new(&item, last_time));
            }
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SettingTemplateModel {
    name: String,
    created: String,
    updated: String,
    #[serde(rename = "lastRequest")]
    last_request: i64,
}

impl SettingTemplateModel {
    pub fn new(itm: &TemplateItem, last_request: i64) -> Self {
        Self {
            name: itm.id.to_string(),
            created: itm.created.to_rfc3339(),
            updated: itm.last_update.to_rfc3339(),
            last_request,
        }
    }
}
