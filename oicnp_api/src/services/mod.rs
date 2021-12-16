pub mod user;
pub mod taxonomies;
pub mod nodes;

pub use user::*;
pub use taxonomies::*;
pub use nodes::*;


use crate::utils::AppConfig;

pub struct ServiceContext {
    pub config: AppConfig,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = AppConfig::default();

        ServiceContext {
            config,
        }
    }
}

lazy_static! {
    pub static ref G: ServiceContext = ServiceContext::default();
}