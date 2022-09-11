use std::sync::Arc;

use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, caches::SecretUsage, my_no_sql::SecretMyNoSqlEntity};

#[derive(MyHttpInput)]
pub struct PostSecretContract {
    #[http_form(description = "Name")]
    pub name: String,
    #[http_form(description = "Secret")]
    pub secret: String,
}

#[derive(MyHttpInput)]
pub struct GetSecretContract {
    #[http_form(description = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfSecretsContract {
    data: Vec<SecretModel>,
}

impl ListOfSecretsContract {
    pub async fn new(app: &AppContext, items: Vec<Arc<SecretMyNoSqlEntity>>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for item in items {
            data.push(SecretModel::new(app, &item).await);
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretModel {
    amount: usize,
    name: String,
    created: String,
    updated: String,
}

impl SecretModel {
    pub async fn new(app: &AppContext, itm: &SecretMyNoSqlEntity) -> Self {
        Self {
            name: itm.row_key.to_string(),
            created: itm.create_date.to_string(),
            updated: itm.last_update_date.to_string(),
            amount: crate::operations::secrets::get_used_secret_amount(app, itm.row_key.as_str())
                .await,
        }
    }
}

// Secret Usage

#[derive(MyHttpInput)]
pub struct ShowUsageInputContract {
    #[http_form(description = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ShowSecretUsageResponse {
    data: Vec<SecretUsageModel>,
}

impl ShowSecretUsageResponse {
    pub fn new(src: Vec<SecretUsage>) -> Self {
        let mut data = Vec::new();

        for itm in src {
            data.push(SecretUsageModel {
                env: itm.env,
                name: itm.name,
                yaml: itm.yaml,
            });
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretUsageModel {
    env: String,
    name: String,
    yaml: String,
}
