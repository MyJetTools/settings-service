use rust_common::placeholders::PlaceholdersIterator;
use rust_extensions::{
    base64::{FromBase64, IntoBase64},
    ShortString,
};
use serde::{Deserialize, Serialize};

use crate::{caches::SecretsSnapshot, models::ProductId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_base_64(&self) -> String {
        self.0.as_bytes().into_base64()
    }

    pub fn from_base_64(src: &str) -> Self {
        let result = src.from_base64().unwrap();
        let value = String::from_utf8(result).unwrap();
        Self(value)
    }

    pub fn has_secret_inside(&self) -> bool {
        crate::scripts::has_secrets_in_content(&self.0)
    }

    pub fn has_missing_placeholders(
        &self,
        product_id: ProductId<'_>,
        secrets: &SecretsSnapshot,
    ) -> bool {
        use rust_common::placeholders::*;
        for token in PlaceholdersIterator::new(
            self.0.as_str(),
            crate::consts::PLACEHOLDER_OPEN,
            crate::consts::PLACEHOLDER_CLOSE,
        ) {
            match token {
                ContentToken::Text(_) => {}
                ContentToken::Placeholder(secret_id) => {
                    if !secrets.has_secret_to_consume(product_id, secret_id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn has_the_secret_inside(&self, secret_id: &str) -> bool {
        let mut secret_string = ShortString::new_empty();
        secret_string.push_str(crate::consts::PLACEHOLDER_OPEN);
        secret_string.push_str(secret_id);
        secret_string.push_str(crate::consts::PLACEHOLDER_CLOSE);

        self.0.contains(secret_string.as_str())
    }

    pub fn get_secrets(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for token in PlaceholdersIterator::new(
            self.0.as_str(),
            crate::consts::PLACEHOLDER_OPEN,
            crate::consts::PLACEHOLDER_CLOSE,
        ) {
            match token {
                rust_common::placeholders::ContentToken::Text(_) => {}
                rust_common::placeholders::ContentToken::Placeholder(secret_name) => {
                    result.push(secret_name)
                }
            }
        }

        result
    }
}

impl Into<Content> for String {
    fn into(self) -> Content {
        Content(self)
    }
}
