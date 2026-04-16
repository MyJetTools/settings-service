use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithStrKey};

use crate::models::Content;

#[derive(Debug, Clone)]
pub struct SecretItem {
    pub id: String,
    pub content: Content,
    pub remote_value: Option<Content>,
    pub level: u8,
    pub created: DateTimeAsMicroseconds,
    pub updated: DateTimeAsMicroseconds,
}

impl SecretItem {
    pub fn resolve_content(&self, is_remote: bool) -> &Content {
        if is_remote {
            if let Some(remote) = self.remote_value.as_ref() {
                if !remote.as_str().is_empty() {
                    return remote;
                }
            }
        }
        &self.content
    }
}

impl EntityWithStrKey for SecretItem {
    fn get_key(&self) -> &str {
        &self.id
    }
}
