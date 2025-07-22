use actix_web::error;
use educe::Educe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Educe)]
#[educe(Default)]
pub struct CommonResponse {
    #[educe(Default = 200)]
    code: u16,

    #[educe(Default=None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
}

impl CommonResponse {
    pub fn new(code: u16) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }

    pub fn with_msg(code: u16, msg: String) -> Self {
        Self {
            code,
            msg: Some(msg),
        }
    }
}

impl<T: error::ResponseError> From<T> for CommonResponse {
    fn from(value: T) -> Self {
        Self {
            code: value.status_code().as_u16(),
            msg: Some(value.to_string()),
        }
    }
}
