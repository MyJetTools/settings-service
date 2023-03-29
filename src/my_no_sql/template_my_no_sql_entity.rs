use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[my_no_sql_macros::my_no_sql_entity("settingstemplate")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateMyNoSqlEntity {
    #[serde(rename = "CreateDate")]
    pub create_date: String,
    #[serde(rename = "LastUpdateDate")]
    pub last_update_date: String,
    #[serde(rename = "SettingsYamlTemplate")]
    pub yaml_template: String,
}

impl TemplateMyNoSqlEntity {
    pub fn update_yaml(&self, yaml: String) -> Self {
        Self {
            yaml_template: yaml,
            last_update_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            ..self.clone()
        }
    }
}
