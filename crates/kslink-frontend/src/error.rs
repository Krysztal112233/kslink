use thiserror::Error;
use url::Url;

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),

    #[error("invalid url: {0}")]
    InvalidUrl(Url),

    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Internal(format!("{value}"))
    }
}
