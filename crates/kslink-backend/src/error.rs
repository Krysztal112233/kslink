use rocket::http::Status;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    UrlParseFailed(#[from] url::ParseError),
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(by) => Error::NotFound(by),
            _ => Error::Internal(value.to_string()),
        }
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound(_) => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }
}

impl From<Error> for u16 {
    fn from(value: Error) -> Self {
        let code: Status = value.into();
        code.code
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(value.to_string())
    }
}
