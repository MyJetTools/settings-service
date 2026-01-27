use std::collections::HashMap;

use rust_extensions::sorted_vec::*;

use crate::models::*;

pub struct TemplatesCacheInner {
    pub items: HashMap<String, SortedVecOfArcWithStrKey<TemplateItem>>,
}

impl Default for TemplatesCacheInner {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
