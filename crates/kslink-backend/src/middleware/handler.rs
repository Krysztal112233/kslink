use rocket::{catch, http::Status, Request};
use serde_json::Value;

use crate::common::response::CommonResponse;

#[catch(default)]
#[tracing::instrument]
pub fn default(status: Status, _req: &Request) -> Value {
    serde_json::to_value(CommonResponse::with_msg(status.code, status.to_string())).unwrap()
}
