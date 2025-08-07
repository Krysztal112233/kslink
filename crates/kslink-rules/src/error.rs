use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Regex(#[from] regex::Error),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
