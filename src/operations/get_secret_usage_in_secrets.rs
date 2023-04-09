use crate::app_ctx::{AppContext, SecretsValueReader};

pub struct SecretSecretUsage {
    pub name: String,
    pub value: String,
}

pub async fn get_secret_usage_in_secrets(
    app: &AppContext,
    secret_name: &str,
) -> Vec<SecretSecretUsage> {
    let secrets = super::get_all_secrets(app).await;

    if secrets.is_none() {
        return Vec::new();
    }

    let secrets = secrets.unwrap();

    let mut result = Vec::with_capacity(secrets.len());

    for secret in secrets {
        if secret
            .get_secret_usages()
            .iter()
            .any(|itm| itm == secret_name)
        {
            if let Some(value) = &secret.value {
                let value = app.get_secret_value(value).await;
                result.push(SecretSecretUsage {
                    name: secret.get_secret_name().to_string(),
                    value: if let Some(value) = value {
                        value.content
                    } else {
                        String::new()
                    },
                });
            }
        }
    }

    result
}
