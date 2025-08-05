use rocket::{State, get};
use sea_orm::DatabaseConnection;

use crate::{cache::CacheImpl, common::response::CommonResponse};

#[get("/")]
pub async fn get_statistics(
    db: &State<DatabaseConnection>,
    mut cache: CacheImpl,
) -> CommonResponse {
    match cache.get_statistics(db.inner()).await {
        Ok(sta) => CommonResponse::default().append_all(sta),
        Err(err) => CommonResponse::from(err),
    }
}
