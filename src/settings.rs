use my_no_sql_sdk::{data_writer::MyNoSqlWriterSettings, reader::MyNoSqlTcpConnectionSettings};
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
    pub my_no_sql_writer: String,
    pub my_no_sql_reader: String,
    pub http_port: u16,

    pub encryption_key: String,

    pub env: String,

    pub favicon_color: Option<String>,

    pub max_level_of_secrets_to_export: u8,
}

#[async_trait::async_trait]
impl MyNoSqlWriterSettings for SettingsModel {
    fn get_app_name(&self) -> &'static str {
        crate::app_ctx::APP_NAME
    }
    fn get_app_version(&self) -> &'static str {
        crate::app_ctx::APP_VERSION
    }
    async fn get_url(&self) -> String {
        self.my_no_sql_writer.clone()
    }
}
#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsModel {
    async fn get_host_port(&self) -> String {
        self.my_no_sql_reader.clone()
    }
}

impl SettingsModel {
    /*
    pub async fn get_key_value_key(&self) -> String {
        return Some(value.to_string());

        //Reading in Docker Swarm case
        let file = tokio::fs::read_to_string("/run/secrets/settings_encryption_key").await;

        if file.is_err() {
            return None;
        }
        let mut result = file.unwrap();
        println!("Got Encryption key from /run/secrets. Len:{}", result.len());

        if result.len() > 48 {
            println!("Truncating key to 48 bytes");
            result.truncate(48);
        }

        Some(result)
    }

     */
    pub fn get_favicon_suffix(&self) -> FaviconColor {
        match self.favicon_color.as_ref() {
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
