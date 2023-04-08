use crate::{app_ctx::SecretsValueReader, caches::SecretValue};

use super::ContentToken;

pub async fn populate_with_secrets(
    secrets_value_reader: &impl SecretsValueReader,
    content_to_populate: &str,
    min_value: Option<u8>,
) -> String {
    if !has_secrets_to_populate(content_to_populate) {
        return content_to_populate.to_string();
    }

    let mut result = if let Some(min_value) = min_value {
        let mut result = String::new();
        for item in super::get_tokens_with_placeholders(content_to_populate) {
            match item {
                ContentToken::Text(text) => {
                    result.push_str(text);
                }
                ContentToken::Placeholder(secret_name) => {
                    result.push_str(secret_name);
                    result.push(':');
                    result.push_str(min_value.to_string().as_str());
                }
            }
        }

        result
    } else {
        content_to_populate.to_string()
    };

    loop {
        result = populate_template_with_secrets(secrets_value_reader, result.as_str()).await;

        if !has_secrets_to_populate(result.as_str()) {
            return result;
        }
    }
}

async fn populate_template_with_secrets(
    secrets_value_reader: &impl SecretsValueReader,
    content_to_populate: &str,
) -> String {
    let tokens = super::get_tokens_with_placeholders(content_to_populate);
    let mut result = String::new();

    for template_token in tokens {
        match template_token {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_name) => {
                let (secret_name, secret_min_level) = parse_secret_line(secret_name);

                match secrets_value_reader.get_secret_value(secret_name).await {
                    Some(secret_value) => {
                        if let Some(secret_min_level) = secret_min_level {
                            if secret_value.level > secret_min_level {
                                if has_secrets_to_populate(&secret_value.value) {
                                    recompile_token(secret_value, &mut result);
                                } else {
                                    result.push_str(secret_value.value.as_str());
                                }
                            } else {
                                result.push_str(&format!(
                                    "/*Secret {} has lower level {} than required {}*/",
                                    secret_name, secret_value.level, secret_min_level
                                ));
                            }
                        } else {
                            if has_secrets_to_populate(&secret_value.value) {
                                recompile_token(secret_value, &mut result);
                            } else {
                                result.push_str(secret_value.value.as_str());
                            }
                        }
                    }
                    None => {
                        result.push_str(&format!("/*Secret {} not found*/", secret_name));
                    }
                }
            }
        }
    }

    result
}

fn has_secrets_to_populate(src: &str) -> bool {
    src.contains("${")
}

fn recompile_token(secret_value: SecretValue, result: &mut String) {
    for secret_token in super::get_tokens_with_placeholders(secret_value.value.as_str()) {
        match secret_token {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_name) => {
                result.push_str("${");
                result.push_str(secret_name);
                result.push_str(":");
                result.push_str(secret_value.level.to_string().as_str());
                result.push_str("}");
            }
        }
    }
}

pub fn parse_secret_line(src: &str) -> (&str, Option<u8>) {
    let src_as_bytes = src.as_bytes();

    let mut i = src.len() - 1;

    while i > 0 {
        if src_as_bytes[i] == b':' {
            break;
        }

        i -= 1;
    }

    if i == 0 {
        return (src, None);
    }

    let secret_name = std::str::from_utf8(&src_as_bytes[..i]).unwrap();

    let min_level_str = std::str::from_utf8(&src_as_bytes[i + 1..]).unwrap();

    println!("min_level_str: {}", min_level_str);
    let min_level = min_level_str.parse::<u8>().ok();

    (secret_name, min_level)
}

#[cfg(test)]
mod test {
    use crate::{app_ctx::SecretsValueReader, caches::SecretValue};

    use super::populate_with_secrets;

    pub struct SecretsValueReaderMock {
        hash_map: std::collections::HashMap<String, SecretValue>,
    }

    impl SecretsValueReaderMock {
        pub fn new() -> Self {
            Self {
                hash_map: std::collections::HashMap::new(),
            }
        }
        pub fn add(&mut self, secret_name: &str, secret_value: SecretValue) {
            self.hash_map.insert(secret_name.to_string(), secret_value);
        }
    }

    #[async_trait::async_trait]
    impl SecretsValueReader for SecretsValueReaderMock {
        async fn get_secret_value(&self, secret_name: &str) -> Option<SecretValue> {
            self.hash_map.get(secret_name).cloned()
        }
    }

    #[test]
    fn parse_secret_line() {
        let secret_line = "secret_name";
        let (secret_name, min_level) = super::parse_secret_line(secret_line);

        assert_eq!(secret_name, "secret_name");
        assert_eq!(min_level, None);
    }

    #[test]
    fn parse_secret_line_with_min_level() {
        let secret_line = "secret_name:5";
        let (secret_name, min_level) = super::parse_secret_line(secret_line);

        assert_eq!(secret_name, "secret_name");
        assert_eq!(min_level, Some(5));
    }

    #[tokio::test]
    async fn test_general_secret_population() {
        let mut secret_value_reader = SecretsValueReaderMock::new();

        secret_value_reader.add(
            "test",
            SecretValue {
                value: "15".to_owned(),
                level: 0,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish", None).await;

        assert_eq!(result, "myData: start15finish");
    }

    #[tokio::test]
    async fn test_general_secret_population_with_sub_secrets() {
        let mut secret_value_reader = SecretsValueReaderMock::new();

        secret_value_reader.add(
            "test",
            SecretValue {
                value: "15${SubSecret}16".to_owned(),
                level: 0,
            },
        );

        secret_value_reader.add(
            "SubSecret",
            SecretValue {
                value: "SubSecData".to_owned(),
                level: 1,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish", None).await;

        assert_eq!(result, "myData: start15SubSecData16finish");
    }

    #[tokio::test]
    async fn test_general_secret_population_with_sub_secrets_with_the_same_level() {
        let mut secret_value_reader = SecretsValueReaderMock::new();

        secret_value_reader.add(
            "test",
            SecretValue {
                value: "15${SubSecret}16".to_owned(),
                level: 0,
            },
        );

        secret_value_reader.add(
            "SubSecret",
            SecretValue {
                value: "SubSecData".to_owned(),
                level: 0,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish", None).await;

        assert_eq!(
            result,
            "myData: start15/*Secret SubSecret has lower level 0 than required 0*/16finish"
        );
    }
}
