use my_no_sql_sdk::data_writer::MyNoSqlWriterSettings;
use serde::{Deserialize, Serialize};

pub enum FaviconColor {
    Default,
    Green,
    Pink,
    Black,
    Yellow,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyNoSqlServer")]
    pub my_no_sql_server: String,
    #[serde(rename = "KeyVaultUrl")]
    pub key_vault_url: Option<String>,
    #[serde(rename = "HttpPort")]
    pub http_port: u16,
    #[serde(rename = "Environment")]
    pub env: String,
    #[serde(rename = "FaviconColour")]
    favicon_colour: Option<String>,
}

#[async_trait::async_trait]
impl MyNoSqlWriterSettings for SettingsModel {
    async fn get_url(&self) -> String {
        self.my_no_sql_server.clone()
    }
}

impl SettingsModel {
    pub fn get_key_value_key(&self) -> Option<String> {
        match std::env::var("KeyValueKey") {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    pub fn get_favicon_suffix(&self) -> FaviconColor {
        match self.favicon_colour.as_ref() {
            Some(suffix) => match suffix.to_lowercase().as_str() {
                "black" => FaviconColor::Black,
                "green" => FaviconColor::Green,
                "pink" => FaviconColor::Pink,
                "yellow" => FaviconColor::Yellow,
                _ => panic!("Unknown favicon suffix: {}", suffix),
            },
            None => {
                println!("Settings.FaviconSuffix is not set. Using default favicon");
                FaviconColor::Default
            }
        }
    }
}
