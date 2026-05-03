use my_http_server::macros::*;
use serde::{Deserialize, Serialize};

use crate::flows::ProductListItem;

#[derive(MyHttpInput)]
pub struct PostProductContract {
    #[http_body(description = "Product id")]
    pub id: String,
    #[http_body(description = "Product description")]
    pub description: String,
    #[http_body(description = "Prompt that explains the product context to AI agents")]
    pub prompt: String,
}

#[derive(MyHttpInput)]
pub struct GetProductContract {
    #[http_query(description = "Product id")]
    pub id: String,
}

#[derive(MyHttpInput)]
pub struct DeleteProductContract {
    #[http_body(description = "Product id")]
    pub id: String,
}

#[derive(MyHttpInput)]
pub struct GetProductsListContract {}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ProductHttpModel {
    pub id: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    #[serde(rename = "templatesAmount")]
    pub templates_amount: i32,
    #[serde(rename = "secretsAmount")]
    pub secrets_amount: i32,
    #[serde(rename = "hasMetadata")]
    pub has_metadata: bool,
}

impl From<ProductListItem> for ProductHttpModel {
    fn from(value: ProductListItem) -> Self {
        Self {
            id: value.id,
            description: value.description,
            prompt: value.prompt,
            templates_amount: value.templates_count,
            secrets_amount: value.secrets_count,
            has_metadata: value.has_metadata,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ListOfProductsContract {
    pub data: Vec<ProductHttpModel>,
}
