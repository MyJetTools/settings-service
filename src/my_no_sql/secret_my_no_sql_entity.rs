use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretMyNoSqlEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "CreateDate")]
    pub create_date: String,
    #[serde(rename = "LastUpdate")]
    pub last_update_date: String,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SecretMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "SettingsSecrets"
    }
}

impl MyNoSqlEntity for SecretMyNoSqlEntity {
    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }
    fn get_row_key(&self) -> &str {
        &self.row_key
    }
    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
