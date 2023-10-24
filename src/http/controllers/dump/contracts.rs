use my_http_server::{macros::MyHttpInput, types::FileContent};

use crate::my_no_sql::TemplateMyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingTemplateDumpModel {
    env: String,
    name: String,
    template: String,
}

impl SettingTemplateDumpModel {
    pub fn new(itm: &TemplateMyNoSqlEntity) -> Self {
        Self {
            env: itm.partition_key.clone(),
            name: itm.row_key.clone(),
            template: itm.yaml_template.clone(),
        }
    }
}

impl Into<TemplateMyNoSqlEntity> for SettingTemplateDumpModel {
    fn into(self) -> TemplateMyNoSqlEntity {
        TemplateMyNoSqlEntity {
            partition_key: self.env,
            row_key: self.name,
            time_stamp: "".to_string(),
            create_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            last_update_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            yaml_template: self.template,
        }
    }
}

#[derive(MyHttpInput)]
pub struct ImportSettingsTemplateAction {
    #[http_form_data(name = "dump", description = "Dump file")]
    pub dump: FileContent,
}
