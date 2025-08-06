use serde::{Deserialize, Serialize};

pub mod url_mapping;
pub mod visitor;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    pub count: u64,
    pub visit: u64,
}
