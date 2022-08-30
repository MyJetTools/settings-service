use std::sync::Arc;

use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::my_no_sql::SecretMyNoSqlEntity;

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
    pub fn new(items: Vec<Arc<SecretMyNoSqlEntity>>) -> Self {
        let mut data = Vec::with_capacity(items.len());

        for item in items {
            data.push(SecretModel::new(&item));
        }

        Self { data }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct SecretModel {
    name: String,
    created: String,
    updated: String,
}

impl SecretModel {
    pub fn new(itm: &SecretMyNoSqlEntity) -> Self {
        Self {
            name: itm.row_key.to_string(),
            created: itm.create_date.to_string(),
            updated: itm.last_update_date.to_string(),
        }
    }
}
