use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub url: Url,
}

impl From<Url> for CreateRequest {
    fn from(url: Url) -> Self {
        Self { url }
    }
}
