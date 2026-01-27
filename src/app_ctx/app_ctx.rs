use std::sync::Arc;

use rust_extensions::{file_utils::FilePath, AppStates};
use tokio::sync::Mutex;

use crate::{caches::*, persistence::*, settings::SettingsModel};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub process_id: String,

    pub settings: Arc<SettingsModel>,

    pub templates: TemplatesCache,
    pub templates_persistence: TemplatesPersistence,

    pub secrets: SecretsCache,
    pub secrets_persistence: SecretsPersistence,

    pub last_time_access: Mutex<LastRequestTimeCache>,
}

impl AppContext {
    pub async fn new(settings: SettingsModel) -> Self {
        let settings = Arc::new(settings.clone());

        let aes_key = encryption::aes::AesKey::new(settings.encryption_key.as_bytes());

        let db_path = FilePath::from_str(&settings.data_path);

        Self {
            settings,
            app_states: Arc::new(AppStates::create_un_initialized()),

            process_id: uuid::Uuid::new_v4().to_string(),

            templates: TemplatesCache::new(),

            templates_persistence: TemplatesPersistence::new(db_path.clone()),

            secrets: SecretsCache::default(),
            secrets_persistence: SecretsPersistence::new(db_path, aes_key),
            last_time_access: Mutex::new(LastRequestTimeCache::new()),
        }
    }
}
