mod export_snapshot;
mod get_domains;

pub use get_domains::*;

mod set_domain_mask;
pub use set_domain_mask::*;

//mod initialize_templates;

pub mod templates;

pub use export_snapshot::*;
//pub use initialize_templates::*;

mod set_domain_product_info;
pub use set_domain_product_info::*;
mod delete_domain_product_info;
pub use delete_domain_product_info::*;
mod import_snapshot;
pub use import_snapshot::*;
//mod init_on_start;
//pub use init_on_start::*;
