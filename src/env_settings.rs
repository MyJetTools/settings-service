pub struct EnvSettings {
    pub azure_client_secret: String,
    pub azure_tenant_id: String,
    pub azure_client_id: String,
}

impl EnvSettings {
    pub fn load() -> Self {
        Self {
            azure_client_secret: get_env_variable_value("AZURE_CLIENT_SECRET"),
            azure_tenant_id: get_env_variable_value("AZURE_TENANT_ID"),
            azure_client_id: get_env_variable_value("AZURE_CLIENT_ID"),
        }
    }
}

fn get_env_variable_value(key: &str) -> String {
    if let Ok(value) = std::env::var(key) {
        return value;
    }

    panic!("Evn variable {} is not found", key);
}
