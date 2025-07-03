use std::sync::Arc;

use my_no_sql_sdk::{
    abstractions::DataSynchronizationPeriod,
    data_writer::{CreateTableParams, MyNoSqlDataWriter},
    reader::{MyNoSqlDataReaderTcp, MyNoSqlTcpConnection},
};
use rust_extensions::AppStates;

use crate::{
    caches::LastRequestTimeCache,
    my_no_sql::{DomainMyNoSqlEntity, SecretMyNoSqlEntity, TemplateMyNoSqlEntity},
    settings::SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub templates_storage: MyNoSqlDataWriter<TemplateMyNoSqlEntity>,
    pub templates_storage_reader: Arc<MyNoSqlDataReaderTcp<TemplateMyNoSqlEntity>>,
    pub domains_setup: MyNoSqlDataWriter<DomainMyNoSqlEntity>,
    pub secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
    pub secrets_storage_reader: Arc<MyNoSqlDataReaderTcp<SecretMyNoSqlEntity>>,

    pub process_id: String,
    //pub templates_cache: TemplatesCache,
    //pub secrets_repository: SecretsRepository,
    pub last_request: LastRequestTimeCache,
    pub aes_key: encryption::aes::AesKey,
    pub settings: Arc<SettingsModel>,
    pub reader_connection: MyNoSqlTcpConnection,
}

impl AppContext {
    pub async fn new(settings: SettingsModel) -> Self {
        let settings = Arc::new(settings.clone());
        let templates_storage = MyNoSqlDataWriter::new(
            settings.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            DataSynchronizationPeriod::Sec5,
        );

        let secrets_storage = MyNoSqlDataWriter::new(
            settings.clone(),
            CreateTableParams {
                persist: true,
                max_partitions_amount: None,
                max_rows_per_partition_amount: None,
            }
            .into(),
            DataSynchronizationPeriod::Sec5,
        );

        let aes_key = encryption::aes::AesKey::new(settings.encryption_key.as_bytes());

        let reader_connection = MyNoSqlTcpConnection::new(APP_NAME, settings.clone());

        /*
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
            SecretsRepository::new(
                KeyValueRepositoryStorage::EncodingKey(aes_key),
                secrets_storage,
            )
        } else {
            panic!("No key vault url or key");
        };
         */

        let domains_setup = MyNoSqlDataWriter::new(
            settings.clone(),
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
            secrets_storage,
            secrets_storage_reader: reader_connection.get_reader().await,
            templates_storage_reader: reader_connection.get_reader().await,

            last_request: LastRequestTimeCache::new(),
            aes_key,

            domains_setup,
            reader_connection,
        }
    }
}
