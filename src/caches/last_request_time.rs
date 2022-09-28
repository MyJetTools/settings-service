use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

pub struct LastRequestTimeCache {
    pub items: Mutex<BTreeMap<String, BTreeMap<String, DateTimeAsMicroseconds>>>,
}

impl LastRequestTimeCache {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn update(&self, env: &str, name: &str, now: DateTimeAsMicroseconds) {
        let mut write_access = self.items.lock().await;

        if !write_access.contains_key(env) {
            write_access.insert(env.to_string(), BTreeMap::new());
        }

        let sub_access = write_access.get_mut(env).unwrap();

        sub_access.insert(name.to_string(), now);
    }

    pub async fn get_snapshot(&self) -> BTreeMap<String, BTreeMap<String, DateTimeAsMicroseconds>> {
        let write_access = self.items.lock().await;
        write_access.clone()
    }
}
