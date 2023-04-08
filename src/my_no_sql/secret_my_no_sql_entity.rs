use serde::*;

use crate::caches::SecretValue;

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
    #[serde(rename = "SecretUsages")]
    pub secret_usages: Option<String>,
}

impl SecretMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "SettingsSecrets"
    }

    pub fn get_secret_name(&self) -> &str {
        self.row_key.as_str()
    }

    pub fn get_level(&self) -> u8 {
        self.level.unwrap_or(0)
    }

    pub fn to_empty_value(&self) -> SecretValue {
        SecretValue {
            value: "".to_string(),
            level: self.get_level(),
        }
    }

    pub fn get_secret_usages(&self) -> Vec<String> {
        let value = self.secret_usages.as_ref();
        if value.is_none() {
            return vec![];
        }

        match serde_json::from_str(value.unwrap()) {
            Ok(result) => result,
            Err(_) => vec![],
        }
    }
}
