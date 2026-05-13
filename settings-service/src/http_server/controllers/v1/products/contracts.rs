use my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, MyHttpObjectStructure)]
pub struct ProductHttpModel {
    pub id: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub templates_amount: i32,
    pub secrets_amount: i32,
    pub has_metadata: bool,
}

#[derive(MyHttpInput)]
pub struct GetProductInput {
    #[http_query(description = "Product id")]
    pub product_id: String,
}

#[derive(MyHttpInput)]
pub struct SaveProductInput {
    #[http_body(description = "Product id")]
    pub id: String,
    #[http_body(description = "Description")]
    pub description: String,
    #[http_body(description = "Prompt for AI agents")]
    pub prompt: String,
}

#[derive(MyHttpInput)]
pub struct DeleteProductInput {
    #[http_body(description = "Product id")]
    pub product_id: String,
}
