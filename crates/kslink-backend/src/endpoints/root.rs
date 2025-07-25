use entity::{
    helper::url_mapping::UrlMappingHelper,
    model::{prelude::*, url_mapping},
};
use rocket::{delete, get, http::Status, post, response::Redirect, serde::json::Json, State};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use tracing::instrument;
use url::Url;

use crate::common::{hasher::ShortHash, response::Either};
use crate::{
    common::{request::CreateRequest, response::CommonResponse},
    error::Error,
};

#[post("/", rank = 1, data = "<form>")]
#[instrument]
pub async fn post_with_json(
    form: Json<CreateRequest>,
    db: &State<DatabaseConnection>,
) -> CommonResponse {
    get_or_create_url(form.0, db.inner()).await
}

#[post("/?<url>", rank = 0)]
#[instrument]
pub async fn post_with_query(url: String, db: &State<DatabaseConnection>) -> CommonResponse {
    match Url::parse(&url).map_err(Error::from) {
        Ok(url) => get_or_create_url(CreateRequest { url }, db.inner()).await,
        Err(err) => err.into(),
    }
}

#[get("/<hash>", rank = 0)]
#[instrument]
pub async fn get_link(
    hash: String,
    db: &State<DatabaseConnection>,
) -> Either<Redirect, CommonResponse> {
    let result = UrlMapping::get_by_hash(hash, db.inner())
        .await
        .map_err(Error::from);

    match result {
        Ok(model) => Either::Left(Redirect::permanent(model.dest)),
        Err(err) => Either::Right(CommonResponse::from(err)),
    }
}

#[delete("/<hash>", rank = 1)]
#[instrument]
pub async fn delete_link(hash: String) {
    todo!()
}

#[get("/<hash>/info")]
#[instrument]
pub async fn get_link_status(hash: String) {
    todo!()
}

async fn get_or_create<C>(c: CreateRequest, db: &C) -> Result<url_mapping::Model, Error>
where
    C: ConnectionTrait,
{
    let result = match UrlMapping::get_by_desc(c.url.clone(), db).await {
        Ok(model) => model,
        Err(_) => UrlMapping::create_short(c.url.clone().into_short_hash(), c.url, db).await?,
    };

    Ok(result)
}

async fn get_or_create_url<C>(c: CreateRequest, db: &C) -> CommonResponse
where
    C: ConnectionTrait,
{
    match get_or_create(c, db).await {
        Ok(model) => CommonResponse::new(Status::Ok.code)
            .append("hash", serde_json::Value::String(model.hash)),
        Err(err) => err.into(),
    }
}
