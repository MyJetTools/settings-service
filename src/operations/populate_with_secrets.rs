use crate::{app_ctx::SecretsValueReader, caches::SecretValue};
use rust_extensions::placeholders::*;

pub async fn populate_with_secrets(
    secrets_value_reader: &impl SecretsValueReader,
    content_to_populate: &str,
) -> String {
    if !has_secrets_to_populate(content_to_populate) {
        return content_to_populate.to_string();
    }

    let mut result = String::new();

    for item in PlaceholdersIterator::new(
        content_to_populate,
        crate::settings_model::PLACEHOLDER_OPEN,
        crate::settings_model::PLACEHOLDER_CLOSE,
    ) {
        match item {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_name) => {
                let secret_value = secrets_value_reader.get_secret_value(secret_name).await;

                if let Some(secret_value) = secret_value {
                    if has_secrets_to_populate(&secret_value.content) {
                        let secret_value =
                            super::populate_secrets_recursively(secrets_value_reader, secret_value)
                                .await;
                        result.push_str(&secret_value);
                    } else {
                        result.push_str(&secret_value.content);
                    }
                } else {
                    populate_secret_not_found(&mut result, secret_name)
                }
            }
        }
    }

    result
}

pub fn populate_secret_not_found(result: &mut String, secret_name: &str) {
    result.push_str("/*Secret ");
    result.push_str(secret_name);
    result.push_str(" is not found*/");
}

pub fn has_secrets_to_populate(src: &str) -> bool {
    src.contains("${")
}

pub fn fill_token_has_wrong_level(
    secret_name: &str,
    secret_value: SecretValue,
    secret_min_level: u8,
    result: &mut String,
) {
    result.push_str(&format!(
        "/*Secret {} has lower level {} than required {}*/",
        secret_name, secret_value.level, secret_min_level
    ));
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
                content: "15".to_owned(),
                level: 0,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish").await;

        assert_eq!(result, "myData: start15finish");
    }

    #[tokio::test]
    async fn test_general_secret_population_with_sub_secrets() {
        let mut secret_value_reader = SecretsValueReaderMock::new();

        secret_value_reader.add(
            "test",
            SecretValue {
                content: "15${SubSecret}16".to_owned(),
                level: 0,
            },
        );

        secret_value_reader.add(
            "SubSecret",
            SecretValue {
                content: "SubSecData".to_owned(),
                level: 1,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish").await;

        assert_eq!(result, "myData: start15SubSecData16finish");
    }

    #[tokio::test]
    async fn test_general_secret_population_with_sub_secrets_with_the_same_level() {
        let mut secret_value_reader = SecretsValueReaderMock::new();

        secret_value_reader.add(
            "test",
            SecretValue {
                content: "15${SubSecret}16".to_owned(),
                level: 0,
            },
        );

        secret_value_reader.add(
            "SubSecret",
            SecretValue {
                content: "SubSecData".to_owned(),
                level: 0,
            },
        );

        let result =
            populate_with_secrets(&secret_value_reader, "myData: start${test}finish").await;

        assert_eq!(
            result,
            "myData: start15/*Secret SubSecret has lower level 0 than required 0*/16finish"
        );
    }
}
