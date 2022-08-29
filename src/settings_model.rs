use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyNoSqlServer")]
    pub my_no_sql_server: String,
    #[serde(rename = "KeyVaultUrl")]
    pub key_vault_url: String,
    #[serde(rename = "HttpPort")]
    pub http_port: u16,
}
