use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::{
    app_ctx::AppContext, caches::SecretUsage, key_value_repository::SecretValue,
    my_no_sql::SecretMyNoSqlEntity,
};

#[derive(MyHttpInput)]
pub struct PostSecretContract {
    #[http_body(description = "Name")]
    pub name: String,
    #[http_body(description = "Secret")]
    pub secret: String,
    #[http_body(description = "Level")]
    pub level: u8,
}

impl Into<SecretValue> for PostSecretContract {
    fn into(self) -> SecretValue {
        SecretValue {
            value: self.secret,
            level: self.level,
        }
    }
}

#[derive(MyHttpInput)]
pub struct GetSecretContract {
    #[http_body(description = "Name")]
    pub name: String,
}
#[derive(Serialize, Debug, MyHttpObjectStructure)]
pub struct SecretHttpModel {
    pub value: String,
    pub level: u8,
}

impl Into<SecretHttpModel> for SecretValue {
    fn into(self) -> SecretHttpModel {
        SecretHttpModel {
            value: self.value,
            level: self.level,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfSecretsContract {
    data: Vec<SecretModel>,
}

impl ListOfSecretsContract {
    pub async fn new(app: &AppContext, items: Vec<SecretMyNoSqlEntity>) -> Self {
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
    level: u8,
}

impl SecretModel {
    pub async fn new(app: &AppContext, itm: &SecretMyNoSqlEntity) -> Self {
        Self {
            name: itm.row_key.to_string(),
            created: itm.create_date.to_string(),
            updated: itm.last_update_date.to_string(),
            amount: crate::operations::secrets::get_used_secret_amount(app, itm.row_key.as_str())
                .await,
            level: itm.level.unwrap_or(0),
        }
    }
}

// Secret Usage

#[derive(MyHttpInput)]
pub struct ShowUsageInputContract {
    #[http_body(description = "Name")]
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

// Delete secret
#[derive(MyHttpInput)]
pub struct DeleteSecretInputContract {
    #[http_body(description = "Name")]
    pub name: String,
}
