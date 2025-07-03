use rust_extensions::placeholders::{ContentToken, PlaceholdersIterator};

#[derive(Debug, Clone)]
pub struct SecretValue {
    pub content: String,
    pub level: u8,
}

impl SecretValue {
    pub fn get_usages(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for token in PlaceholdersIterator::new(
            &self.content,
            crate::scripts::PLACEHOLDER_OPEN,
            crate::scripts::PLACEHOLDER_CLOSE,
        ) {
            match token {
                ContentToken::Text(_) => {}
                ContentToken::Placeholder(secret_name) => result.push(secret_name),
            }
        }

        result
    }
}
