use std::sync::Arc;

use my_azure_key_vault::{BearerTokenManager, MyAzureKeyVault};
use my_no_sql_data_writer::MyNoSqlDataWriter;
use rust_extensions::AppStates;

use crate::{
    caches::{SecretsCache, SecretsValuesCache, TemplatesCache},
    env_settings::EnvSettings,
    my_no_sql::{SecretMyNoSqlEntity, TemplateMyNoSqlEntity},
    settings_model::SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub settings: SettingsModel,
    pub app_states: Arc<AppStates>,
    pub templates_storage: MyNoSqlDataWriter<TemplateMyNoSqlEntity>,
    pub secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
    pub process_id: String,
    pub key_vault_client: MyAzureKeyVault,
    pub secrets_values_cache: SecretsValuesCache,
    pub templates_cache: TemplatesCache,
    pub secrets_cache: SecretsCache,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        let templates_storage = MyNoSqlDataWriter::new(
            settings.my_no_sql_server.clone(),
            "settingstemplate".to_string(),
            true,
            true,
            my_no_sql_server_abstractions::DataSyncronizationPeriod::Sec5,
        );

        let secrets_storage = MyNoSqlDataWriter::new(
            settings.my_no_sql_server.clone(),
            "settingssecrets".to_string(),
            true,
            true,
            my_no_sql_server_abstractions::DataSyncronizationPeriod::Sec5,
        );

        let env_settings = EnvSettings::load();

        let token_manager = BearerTokenManager::new(
            env_settings.azure_tennant_id,
            env_settings.azure_client_id,
            env_settings.azure_client_secret,
        );

        Self {
            key_vault_client: MyAzureKeyVault::new(
                Arc::new(token_manager),
                settings.key_vault_url.clone(),
            ),
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
            templates_storage,
            process_id: uuid::Uuid::new_v4().to_string(),
            secrets_storage,
            secrets_values_cache: SecretsValuesCache::new(),
            templates_cache: TemplatesCache::new(),
            secrets_cache: SecretsCache::new(),
        }
    }
}
