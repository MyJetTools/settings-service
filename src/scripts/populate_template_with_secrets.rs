use crate::{app_ctx::AppContext, models::*};
use rust_common::placeholders::*;

pub async fn populate_template_with_secrets(
    app: &AppContext,
    product_id: &str,
    content: &Content,
) -> Content {
    if !crate::scripts::has_secrets_in_content(content_to_populate) {
        return content_to_populate.to_string();
    }

    let mut result = String::new();

    for item in PlaceholdersIterator::new(
        content_to_populate,
        crate::consts::PLACEHOLDER_OPEN,
        crate::consts::PLACEHOLDER_CLOSE,
    ) {
        match item {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_id) => {
                if secret_id.starts_with('$') {
                    result.push_str("${");
                    result.push_str(&secret_id[1..]);
                    result.push('}');
                } else {
                    let secret_item = app
                        .secrets
                        .get_secret_with_shared(product_id, secret_id)
                        .await;

                    if let Some(secret_item) = secret_item {
                        if secret_item.content {
                            let secret_value = super::populate_secrets_recursively(
                                app,
                                product_id,
                                secret_item.as_ref(),
                            )
                            .await;
                            result.push_str(&secret_value);
                        } else {
                            result.push_str(&secret_item.content);
                        }
                    } else {
                        populate_secret_not_found(&mut result, secret_id)
                    }
                }
            }
        }
    }

    result
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

/*
#[cfg(test)]
mod test {
    use crate::caches::SecretValue;

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
            "myData: start15/*Secret SubSecret has lower level 0 than required 0*/
16finish"
        );
    }
}
*/
