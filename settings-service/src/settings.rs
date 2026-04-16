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
    pub http_port: Option<u16>,
    pub grpc_port: Option<u16>,
    pub encryption_key: String,
    pub env: String,
    pub favicon_color: Option<String>,
    pub max_level_of_secrets_to_export: u8,
    pub data_path: String,
    pub local_env_prefixes: Option<Vec<String>>,
}

impl SettingsModel {
    pub fn is_local_env(&self, env_info: Option<&str>) -> bool {
        let Some(env_info) = env_info else {
            return true;
        };
        let Some(prefixes) = self.local_env_prefixes.as_ref() else {
            return true;
        };
        prefixes.iter().any(|p| env_info.starts_with(p.as_str()))
    }

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

    pub fn get_http_port(&self) -> u16 {
        match self.http_port {
            Some(port) => port,
            None => 8000,
        }
    }

    pub fn get_grpc_port(&self) -> u16 {
        match self.grpc_port {
            Some(port) => port,
            None => 8888,
        }
    }
}
