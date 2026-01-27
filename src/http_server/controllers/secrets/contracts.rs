use std::sync::Arc;

use my_http_server::macros::*;
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

use crate::models::*;
use crate::secrets_grpc::TemplateUsageGrpcModel;

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
pub struct SecretHttpModel {
    pub value: String,
    pub level: u8,
}

impl Into<SecretHttpModel> for SecretItem {
    fn into(self) -> SecretHttpModel {
        SecretHttpModel {
            value: self.content.into_string(),
            level: self.level,
        }
    }
}

impl Into<SecretHttpModel> for &'_ SecretItem {
    fn into(self) -> SecretHttpModel {
        SecretHttpModel {
            value: self.content.to_string(),
            level: self.level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfSecretsContract {
    data: Vec<SecretModel>,
}

impl ListOfSecretsContract {
    pub async fn new(
        app: &AppContext,
        product_id: ProductId<'_>,
        items: Vec<Arc<SecretItem>>,
    ) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for item in items {
            data.push(SecretModel::new(app, product_id, &item).await);
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretModel {
    #[serde(rename = "templatesAmount")]
    templates_amount: usize,
    #[serde(rename = "secretsAmount")]
    secrets_amount: usize,
    name: String,
    created: String,
    updated: String,
    level: u8,
}

impl SecretModel {
    pub async fn new(app: &AppContext, product_id: ProductId<'_>, secret: &SecretItem) -> Self {
        let templates_amount = match product_id {
            ProductId::Shared => 0,
            ProductId::Id(product_id) => {
                app.templates
                    .get_count(product_id, |item| {
                        item.content.has_the_secret_inside(&secret.id)
                    })
                    .await
            }
        };

        let secrets = app.secrets.get_snapshot().await;

        let secrets_amount = secrets.get_count(product_id, |itm| {
            itm.content.has_the_secret_inside(&secret.id)
        });

        Self {
            name: secret.id.to_string(),
            created: secret.created.to_string(),
            updated: secret.updated.to_string(),
            templates_amount,
            secrets_amount,
            level: secret.level,
        }
    }
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
