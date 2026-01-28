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
