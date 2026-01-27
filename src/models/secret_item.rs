use rust_common::placeholders::*;
use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithStrKey};

use crate::models::Content;

#[derive(Debug, Clone)]
pub struct SecretItem {
    pub id: String,
    pub content: Content,
    pub level: u8,
    pub created: DateTimeAsMicroseconds,
    pub updated: DateTimeAsMicroseconds,
}

impl EntityWithStrKey for SecretItem {
    fn get_key(&self) -> &str {
        &self.id
    }
}

impl SecretItem {
    pub fn get_usages(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for token in PlaceholdersIterator::new(
            self.content.as_str(),
            crate::consts::PLACEHOLDER_OPEN,
            crate::consts::PLACEHOLDER_CLOSE,
        ) {
            match token {
                ContentToken::Text(_) => {}
                ContentToken::Placeholder(secret_name) => result.push(secret_name),
            }
        }

        result
    }
}
