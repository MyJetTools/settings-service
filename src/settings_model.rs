use serde::{Deserialize, Serialize};

pub enum FaviconColour {
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
    #[serde(rename = "Environment")]
    pub env: String,
    #[serde(rename = "FaviconColour")]
    favicon_colour: Option<String>,
}

impl SettingsModel {
    pub fn get_favicon_suffix(&self) -> FaviconColour {
        match self.favicon_colour.as_ref() {
            Some(suffix) => match suffix.to_lowercase().as_str() {
                "black" => FaviconColour::Black,
                "green" => FaviconColour::Green,
                "pink" => FaviconColour::Pink,
                _ => panic!("Unknown favicon suffix: {}", suffix),
            },
            None => {
                println!("Settings.FaviconSuffix is not set. Using default favicon");
                FaviconColour::Default
            }
        }
    }
}
