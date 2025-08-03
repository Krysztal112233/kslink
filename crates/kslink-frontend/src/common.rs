use std::str::FromStr;

use url::Url;

pub const BASE_URL: &str = env!("KSLINK_BASE_URL");
pub const BUILD_TIME: &str = env!("BUILD_TIME");

pub fn is_valid_url<T>(input: T) -> bool
where
    T: ToString,
{
    Url::from_str(&input.to_string()).is_ok()
}
