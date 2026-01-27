use my_http_server::macros::*;
use serde::{Deserialize, Serialize};

use crate::models::*;
use crate::secrets_grpc::{SecretGrpcModel, TemplateUsageGrpcModel};

#[derive(MyHttpInput)]
pub struct PostSecretContract {
    #[http_query(description = "Product")]
    pub product: Option<String>,
    #[http_body(description = "Name")]
    pub name: String,
    #[http_body(description = "Secret")]
    pub secret: String,
    #[http_body(description = "Level")]
    pub level: u8,
}

#[derive(MyHttpInput)]
pub struct GetSecretContract {
    #[http_query(description = "Product")]
    pub product: Option<String>,
    #[http_body(description = "Name")]
    pub name: String,
}
#[derive(Serialize, Debug, MyHttpObjectStructure)]
pub struct SecretValueHttpModel {
    pub value: String,
    pub level: u8,
}

impl Into<SecretValueHttpModel> for SecretItem {
    fn into(self) -> SecretValueHttpModel {
        SecretValueHttpModel {
            value: self.content.into_string(),
            level: self.level,
        }
    }
}

impl Into<SecretValueHttpModel> for &'_ SecretItem {
    fn into(self) -> SecretValueHttpModel {
        SecretValueHttpModel {
            value: self.content.to_string(),
            level: self.level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfSecretsContract {
    data: Vec<SecretHttpModel>,
}

impl ListOfSecretsContract {
    pub fn new(items: Vec<SecretGrpcModel>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for item in items {
            data.push(SecretHttpModel {
                templates_amount: item.used_by_templates,
                secrets_amount: item.used_by_secrets,
                name: item.secret_id,
                created: item.created,
                updated: item.updated,
                level: item.level,
            });
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretHttpModel {
    #[serde(rename = "templatesAmount")]
    pub templates_amount: i32,
    #[serde(rename = "secretsAmount")]
    pub secrets_amount: i32,
    pub name: String,
    pub created: String,
    pub updated: String,
    pub level: i32,
}

// Secret Usage

#[derive(MyHttpInput)]
pub struct ShowUsageInputContract {
    #[http_query(description = "Product")]
    pub product: String,
    #[http_body(description = "Id of secret")]
    pub secret: String,
}

#[derive(MyHttpInput)]
pub struct ShowSecretesUsageInputContract {
    #[http_query(description = "Product")]
    pub product: Option<String>,
    #[http_body(description = "Name")]
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ShowSecretUsageHttpResponse {
    data: Vec<SecretUsageHttpModel>,
}

impl ShowSecretUsageHttpResponse {
    pub fn new(src: Vec<TemplateUsageGrpcModel>) -> Self {
        let mut data = Vec::with_capacity(src.len());

        for itm in src {
            data.push(SecretUsageHttpModel {
                template_id: itm.template_id,
                template_content: itm.template_content,
            });
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretUsageHttpModel {
    template_id: String,
    template_content: String,
}

// Delete secret
#[derive(MyHttpInput)]
pub struct DeleteSecretInputContract {
    #[http_query(description:"Environment")]
    pub product: Option<String>,
    #[http_body(description = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretSecretUsageHttpModel {
    pub name: String,
    pub value: String,
}

#[derive(MyHttpInput)]
pub struct GenerateRandomSecretContract {
    #[http_query(description: "Environment")]
    pub product: Option<String>,
    #[http_body(description = "Name")]
    pub name: String,
    #[http_body(description = "Level")]
    pub level: u8,
    #[http_body(description = "Length")]
    pub length: usize,

    #[http_body(description = "Force")]
    pub force: Option<bool>,
}

impl GenerateRandomSecretContract {
    pub fn has_force_update(&self) -> bool {
        if let Some(force) = self.force {
            return force;
        }

        false
    }
}
