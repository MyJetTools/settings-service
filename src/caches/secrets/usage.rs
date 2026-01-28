use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Usage {
    items: HashMap<String, usize>,
}

impl Usage {
    pub fn reset(&mut self) {
        self.items.clear();
    }

    pub fn inc(&mut self, secret_id: &str) {
        match self.items.get_mut(secret_id) {
            Some(value) => *value += 1,
            None => {
                self.items.insert(secret_id.to_string(), 1);
            }
        }
    }

    pub fn get(&self, secret_id: &str) -> usize {
        if let Some(value) = self.items.get(secret_id) {
            return *value;
        }

        0
    }
}
