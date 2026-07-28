use dioxus_utils::*;

use crate::models::*;

#[derive(Debug, Clone, Default)]
pub struct ProductValue {
    pub description: String,
    pub prompt: String,
}

pub struct EditProductState {
    pub product_id: String,
    pub value: ProductValue,
    pub value_on_init: DataState<ProductValue>,
    pub new_product: bool,
}

impl EditProductState {
    pub fn new(product_id: String) -> Self {
        let new_product = product_id.is_empty();

        let value = ProductValue::default();

        let value_on_init = if new_product {
            DataState::new_as_loaded(value.clone())
        } else {
            DataState::new()
        };

        Self {
            new_product,
            product_id,
            value_on_init,
            value,
        }
    }

    pub fn init_value(&mut self, value: ProductValue) {
        self.value = value.clone();
        self.value_on_init.set_loaded(value);
    }

    pub fn can_be_saved(&self) -> bool {
        if self.product_id.trim().is_empty() {
            return false;
        }

        let value_on_init = match self.value_on_init.as_ref() {
            RenderState::Loaded(value) => value,
            _ => {
                return false;
            }
        };

        if !self.new_product
            && self.value.description == value_on_init.description
            && self.value.prompt == value_on_init.prompt
        {
            return false;
        }

        true
    }

    pub fn get_result(&self) -> UpdateProductHttpModel {
        UpdateProductHttpModel {
            id: self.product_id.clone(),
            description: self.value.description.clone(),
            prompt: self.value.prompt.clone(),
        }
    }

    pub fn save_button_is_disabled(&self) -> bool {
        !self.can_be_saved()
    }
}
