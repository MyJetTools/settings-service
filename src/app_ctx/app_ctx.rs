use std::sync::Arc;

use my_azure_key_vault::{BearerTokenManager, MyAzureKeyVault};
use my_no_sql_sdk::{
    abstractions::DataSynchronizationPeriod,
    data_writer::{CreateTableParams, MyNoSqlDataWriter},
};
use rust_extensions::AppStates;

use crate::{
    caches::{LastRequestTimeCache, SecretsValuesCache, TemplatesCache},
    env_settings::EnvSettings,
    key_value_repository::{KeyValueRepositoryStorage, SecretsRepository},
    my_no_sql::{DomainMyNoSqlEntity, TemplateMyNoSqlEntity},
    settings_model::SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub settings: SettingsModel,

    pub secret_values_cache: SecretsValuesCache,
    pub app_states: Arc<AppStates>,
    pub templates_storage: MyNoSqlDataWriter<TemplateMyNoSqlEntity>,

    pub domains_setup: MyNoSqlDataWriter<DomainMyNoSqlEntity>,

    pub process_id: String,
    pub templates_cache: TemplatesCache,
    pub secrets_repository: SecretsRepository,
    pub last_request: LastRequestTimeCache,
}

impl AppContext {
    pub async fn new(settings: SettingsModel) -> Self {
        let settings_ark = Arc::new(settings.clone());
        let templates_storage = MyNoSqlDataWriter::new(
            settings_ark.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            DataSynchronizationPeriod::Sec5,
        );

        let secrets_storage = MyNoSqlDataWriter::new(
            settings_ark.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            DataSynchronizationPeriod::Sec5,
        );

        let secrets_repository = if let Some(key_value_url) = &settings.key_vault_url {
            let env_settings = EnvSettings::load();

            let token_manager = BearerTokenManager::new(
                env_settings.azure_tenant_id,
                env_settings.azure_client_id,
                env_settings.azure_client_secret,
            );
            let key_vault_client =
                MyAzureKeyVault::new(Arc::new(token_manager), key_value_url.to_string());

            SecretsRepository::new(
                KeyValueRepositoryStorage::KeyValue(key_vault_client),
                secrets_storage,
            )
        } else if let Some(key_value_key) = settings.get_key_value_key().await {
            let aes_key = encryption::aes::AesKey::new(key_value_key.as_bytes());
            SecretsRepository::new(
                KeyValueRepositoryStorage::EncodingKey(aes_key),
                secrets_storage,
            )
        } else {
            panic!("No key vault url or key");
        };

        let domains_setup = MyNoSqlDataWriter::new(
            settings_ark.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            DataSynchronizationPeriod::Sec5,
        );

        Self {
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
            templates_storage,
            process_id: uuid::Uuid::new_v4().to_string(),
            secrets_repository,
            templates_cache: TemplatesCache::new(),
            last_request: LastRequestTimeCache::new(),

            secret_values_cache: SecretsValuesCache::new(),
            domains_setup,
        }
    }
}
