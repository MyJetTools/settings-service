use serde::*;

pub enum FaviconColor {
    Default,
    Green,
    Pink,
    Black,
    Yellow,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub http_port: u16,
    pub encryption_key: String,
    pub env: String,
    pub favicon_color: Option<String>,
    pub max_level_of_secrets_to_export: u8,
    pub db_path: String,
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
