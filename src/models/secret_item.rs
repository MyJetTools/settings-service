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
    pub description: Option<String>,
    pub visible_for_mcp: bool,
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

    /// Ids of the secrets this secret references as `${secret_id}` inside its
    /// value or its remote value - deduplicated, in order of first appearance.
    pub fn get_referenced_secrets(&self) -> Vec<&str> {
        let mut result = Vec::new();

        let remote_value_secrets = self
            .remote_value
            .iter()
            .flat_map(|remote_value| remote_value.get_secrets());

        for secret_id in self
            .content
            .get_secrets()
            .into_iter()
            .chain(remote_value_secrets)
        {
            // `${$name}` is an escaped placeholder, rendered as a literal `${name}`
            if secret_id.starts_with('$') {
                continue;
            }

            if !result.contains(&secret_id) {
                result.push(secret_id);
            }
        }

        result
    }
}

impl EntityWithStrKey for SecretItem {
    fn get_key(&self) -> &str {
        &self.id
    }
}
