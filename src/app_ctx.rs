use std::sync::Arc;

use my_azure_key_vault::{BearerTokenManager, MyAzureKeyVault};
use my_no_sql_data_writer::{CreateTableParams, MyNoSqlDataWriter};
use rust_extensions::AppStates;

use crate::{
    caches::{LastRequestTimeCache, TemplatesCache},
    env_settings::EnvSettings,
    key_value_repository::{KeyValueRepository, KeyValueRepositoryStorage},
    my_no_sql::TemplateMyNoSqlEntity,
    settings_model::SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub settings: SettingsModel,
    pub app_states: Arc<AppStates>,
    pub templates_storage: MyNoSqlDataWriter<TemplateMyNoSqlEntity>,
    pub process_id: String,
    pub templates_cache: TemplatesCache,
    pub key_value_repository: KeyValueRepository,
    pub last_request: LastRequestTimeCache,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        let templates_storage = MyNoSqlDataWriter::new(
            settings.my_no_sql_server.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            my_no_sql_server_abstractions::DataSynchronizationPeriod::Sec5,
        );

        let secrets_storage = MyNoSqlDataWriter::new(
            settings.my_no_sql_server.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            my_no_sql_server_abstractions::DataSynchronizationPeriod::Sec5,
        );

        let key_value_repository = if let Some(key_value_url) = &settings.key_vault_url {
            let env_settings = EnvSettings::load();

            let token_manager = BearerTokenManager::new(
                env_settings.azure_tennant_id,
                env_settings.azure_client_id,
                env_settings.azure_client_secret,
            );
            let key_vault_client =
                MyAzureKeyVault::new(Arc::new(token_manager), key_value_url.to_string());

            KeyValueRepository::new(
                KeyValueRepositoryStorage::KeyValue(key_vault_client),
                secrets_storage,
            )
        } else if let Some(key_value_key) = &settings.key_vault_key {
            let aes_key = encryption::aes::AesKey::new(key_value_key.as_bytes());
            KeyValueRepository::new(
                KeyValueRepositoryStorage::EncodingKey(aes_key),
                secrets_storage,
            )
        } else {
            panic!("No key vault url or key");
        };

        Self {
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
            templates_storage,
            process_id: uuid::Uuid::new_v4().to_string(),
            key_value_repository,
            templates_cache: TemplatesCache::new(),
            last_request: LastRequestTimeCache::new(),
        }
    }
}
