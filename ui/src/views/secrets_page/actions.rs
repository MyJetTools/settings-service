use std::{collections::HashMap, rc::Rc};

use crate::models::*;

const MAX_SECRET_LEVEL: i32 = u8::MAX as i32;

const SHARED_SCOPE_NAME: &str = "Shared";

/// Secrets shown on the page (the selected product + Shared) indexed by scope
pub struct SecretsLookup<'s> {
    product: HashMap<&'s str, &'s SecretHttpModel>,
    shared: HashMap<&'s str, &'s SecretHttpModel>,
}

impl<'s> SecretsLookup<'s> {
    pub fn new(secrets: &'s [SecretHttpModel]) -> Self {
        let mut product = HashMap::new();
        let mut shared = HashMap::new();

        for itm in secrets {
            if itm.product_id.is_some() {
                product.insert(itm.secret_id.as_str(), itm);
            } else {
                shared.insert(itm.secret_id.as_str(), itm);
            }
        }

        Self { product, shared }
    }

    /// Finds a secret the same way a `${secret_id}` placeholder is resolved:
    /// the product scope first, then Shared
    pub fn resolve(&self, secret_id: &str) -> Option<&'s SecretHttpModel> {
        if let Some(result) = self.product.get(secret_id) {
            return Some(result);
        }

        self.shared.get(secret_id).copied()
    }

    /// The product secret which has the same name as the shared one and takes
    /// priority over it when a placeholder is resolved
    pub fn get_overriding(&self, used: &SecretHttpModel) -> Option<&'s SecretHttpModel> {
        if used.product_id.is_some() {
            return None;
        }

        self.product.get(used.secret_id.as_str()).copied()
    }
}

pub enum UsedSecretState<'s> {
    Ok(&'s SecretHttpModel),
    WrongLevel(&'s SecretHttpModel),
    NotFound,
}

/// A secret can use a secret of a higher level than its own - the product one
/// or the shared one
pub fn get_used_secret_state<'s>(
    lookup: &SecretsLookup<'s>,
    itm: &SecretHttpModel,
    used_secret_id: &str,
) -> UsedSecretState<'s> {
    let product = lookup.product.get(used_secret_id).copied();
    let shared = lookup.shared.get(used_secret_id).copied();

    for used in [product, shared].into_iter().flatten() {
        if used.level > itm.level {
            return UsedSecretState::Ok(used);
        }
    }

    match lookup.resolve(used_secret_id) {
        Some(used) => UsedSecretState::WrongLevel(used),
        None => UsedSecretState::NotFound,
    }
}

/// The lowest level a secret must have to be used by `itm`
pub fn get_level_to_be_used_by(itm: &SecretHttpModel) -> i32 {
    (itm.level + 1).min(MAX_SECRET_LEVEL)
}

pub fn get_scope_name(product_id: Option<&str>) -> String {
    match product_id {
        Some(product_id) => format!("product {}", product_id),
        None => SHARED_SCOPE_NAME.to_string(),
    }
}

/// Move of a secret between a product scope and Shared (`None`)
#[derive(Clone)]
pub struct SecretMove {
    pub secret_id: Rc<String>,
    pub from: Option<Rc<String>>,
    pub to: Option<Rc<String>>,
}

impl SecretMove {
    /// A product secret moves to Shared, a shared one - to the selected product.
    /// Returns the reason as an error when the secret can not be moved
    pub fn new(
        lookup: &SecretsLookup,
        itm: &SecretHttpModel,
        selected_product_id: &Rc<String>,
    ) -> Result<Self, String> {
        let secret_id = itm.secret_id.as_str();

        let (from, to) = match itm.product_id.as_ref() {
            Some(product_id) => {
                if lookup.shared.contains_key(secret_id) {
                    return Err("Shared already has a secret with this name".to_string());
                }

                (Some(Rc::new(product_id.to_string())), None)
            }
            None => {
                // a product named as the shared scope is not a scope to move a shared secret to
                if selected_product_id.is_empty()
                    || selected_product_id.eq_ignore_ascii_case(SHARED_SCOPE_NAME)
                {
                    return Err("Select a product to move the secret to".to_string());
                }

                if lookup.product.contains_key(secret_id) {
                    return Err(format!(
                        "Product {} already has a secret with this name",
                        selected_product_id
                    ));
                }

                (None, Some(selected_product_id.clone()))
            }
        };

        Ok(Self {
            secret_id: Rc::new(secret_id.to_string()),
            from,
            to,
        })
    }

    pub fn get_title(&self) -> String {
        format!("Move to {}", self.get_to_name())
    }

    pub fn get_confirmation(&self) -> String {
        format!(
            "Move secret {} from {} to {}?",
            self.secret_id,
            get_scope_name(self.from.as_ref().map(|itm| itm.as_str())),
            self.get_to_name()
        )
    }

    pub fn get_breaks_references_confirmation(&self, result: &MoveSecretApiModel) -> String {
        let mut out = format!(
            "Moving secret {} to {} breaks references.\n",
            self.secret_id,
            self.get_to_name()
        );

        if !result.broken_dependencies.is_empty() {
            out.push_str("\nSecrets it uses will not be found from there:\n");
            for secret_id in &result.broken_dependencies {
                out.push_str(&format!("- {}\n", secret_id));
            }
        }

        if !result.broken_consumers.is_empty() {
            out.push_str("\nIt will not be found by:\n");
            for consumer in &result.broken_consumers {
                out.push_str(&format!(
                    "- {} {}/{}\n",
                    consumer.kind,
                    consumer.product_id.as_deref().unwrap_or(SHARED_SCOPE_NAME),
                    consumer.id
                ));
            }
        }

        out.push_str("\nMove anyway?");

        out
    }

    fn get_to_name(&self) -> String {
        get_scope_name(self.to.as_ref().map(|itm| itm.as_str()))
    }
}
