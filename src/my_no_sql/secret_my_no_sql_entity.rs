use serde::*;

#[my_no_sql_macros::my_no_sql_entity("settingssecrets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretMyNoSqlEntity {
    #[serde(rename = "CreateDate")]
    pub create_date: String,
    #[serde(rename = "LastUpdate")]
    pub last_update_date: String,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Level")]
    pub level: Option<u8>,
}

impl SecretMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "SettingsSecrets"
    }
}
