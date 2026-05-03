use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductHttpModel {
    pub id: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub templates_amount: i32,
    pub secrets_amount: i32,
    pub has_metadata: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductHttpModel {
    pub id: String,
    pub description: String,
    pub prompt: String,
}
