mod populate_with_secrets;
pub mod secrets;
pub mod templates;
pub use populate_with_secrets::*;
mod populate_secrets_recursively;
pub use populate_secrets_recursively::*;

pub const PLACEHOLDER_OPEN: &'static str = "${";
pub const PLACEHOLDER_CLOSE: &'static str = "}";