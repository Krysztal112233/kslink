use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    DbErr(#[from] sea_orm::DbErr),

    #[error("primary key:{0} not found")]
    PKNotFound(String),

    #[error("desc:{0} not found")]
    DestNotFound(String),
}
