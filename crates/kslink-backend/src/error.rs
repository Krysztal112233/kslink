use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Db(#[from] sea_orm::DbErr),
}
