use my_http_server::macros::*;
use serde::{Deserialize, Serialize};

use crate::templates_grpc::TemplateListItemGrpcModel;

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
    pub fn new(items: Vec<TemplateListItemGrpcModel>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for itm in items {
            data.push(SettingTemplateModel {
                name: itm.template_id,
                created: itm.created,
                updated: itm.updated,
                last_request: itm.last_requests,
            });
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
