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
