use serde::{Deserialize, Serialize};

pub enum FaviconSuffix {
    Default,
    Green,
    Pink,
    Black,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyNoSqlServer")]
    pub my_no_sql_server: String,
    #[serde(rename = "KeyVaultUrl")]
    pub key_vault_url: Option<String>,
    #[serde(rename = "HttpPort")]
    pub http_port: u16,
    #[serde(rename = "KeyVaultKey")]
    pub key_vault_key: Option<String>,
    #[serde(rename = "Suffix")]
    pub suffix: String,
    #[serde(rename = "FaviconSuffix")]
    favicon_suffix: Option<String>,
}

impl SettingsModel {
    pub fn get_favicon_suffix(&self) -> FaviconSuffix {
        match self.favicon_suffix.as_ref() {
            Some(suffix) => match suffix.to_lowercase().as_str() {
                "black" => FaviconSuffix::Black,
                "green" => FaviconSuffix::Green,
                "pink" => FaviconSuffix::Pink,
                _ => panic!("Unknown favicon suffix: {}", suffix),
            },
            None => {
                println!("Settings.FaviconSuffix is not set. Using default favicon");
                FaviconSuffix::Default
            }
        }
    }
}
