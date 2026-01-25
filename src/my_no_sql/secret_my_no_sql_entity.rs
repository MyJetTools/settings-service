use my_no_sql_sdk::macros::my_no_sql_entity;
use serde::*;

/* cspell: disable-next-line */
#[my_no_sql_entity("settingssecrets")]
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
    pub const DEFAULT_PARTITION_KEY: &'static str = "SettingsSecrets";

    pub fn get_secret_name(&self) -> &str {
        self.row_key.as_str()
    }

    pub fn get_level(&self) -> u8 {
        self.level.unwrap_or(0)
    }

    /*
       pub fn to_empty_value(&self) -> SecretValue {
           SecretValue {
               content: "".to_string(),
               level: self.get_level(),
           }
       }
    */
    pub fn get_secret_usages(&self) -> Vec<String> {
        match &self.secret_usages {
            Some(value) => match serde_json::from_str(value) {
                Ok(result) => result,
                Err(_) => vec![],
            },
            None => vec![],
        }
    }
}
