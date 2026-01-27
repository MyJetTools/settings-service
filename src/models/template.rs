use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithStrKey};

use crate::models::Content;

pub struct TemplateItem {
    pub id: String,
    pub content: Content,
    pub created: DateTimeAsMicroseconds,
    pub last_update: DateTimeAsMicroseconds,
}

impl EntityWithStrKey for TemplateItem {
    fn get_key(&self) -> &str {
        &self.id
    }
}
