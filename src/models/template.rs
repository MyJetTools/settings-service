use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithStrKey};

use crate::{models::Content, persistence::TemplatePersistenceItem};

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

impl TemplatePersistenceItem for TemplateItem {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }
}
