use rocket::{Request, catch, http::Status};
use serde_json::Value;
use tracing::instrument;

use crate::common::response::CommonResponse;

#[catch(default)]
#[instrument]
pub fn default(status: Status, _req: &Request) -> Value {
    serde_json::to_value(CommonResponse::with_msg(status.code, status.to_string())).unwrap()
}
