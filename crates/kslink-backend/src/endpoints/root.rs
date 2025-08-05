use entity::{
    helper::url_mapping::UrlMappingHelper,
    model::{prelude::*, url_mapping},
};
use rocket::{
    State, delete, get, http::Status, options, post, response::Redirect, serde::json::Json, tokio,
};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use tracing::instrument;
use url::Url;

use crate::{
    cache::CacheImpl,
    common::{hasher::ShortHash, response::Either},
};
use crate::{
    common::{request::CreateRequest, response::CommonResponse},
    error::Error,
};

#[options("/<_..>")]
pub async fn all_options() {}

#[post("/", rank = 1, data = "<form>")]
#[instrument]
pub async fn post_with_json(
    form: Json<CreateRequest>,
    db: &State<DatabaseConnection>,
    cache: CacheImpl,
) -> CommonResponse {
    get_or_create_url(form.0, db.inner(), cache).await
}

#[post("/?<url>", rank = 0)]
#[instrument]
pub async fn post_with_query(
    url: &str,
    db: &State<DatabaseConnection>,
    cache: CacheImpl,
) -> CommonResponse {
    match Url::parse(url).map_err(Error::from) {
        Ok(url) => get_or_create_url(CreateRequest { url }, db.inner(), cache).await,
        Err(err) => err.into(),
    }
}

#[get("/<hash>", rank = 0)]
#[instrument]
pub async fn get_link(
    hash: &str,
    db: &State<DatabaseConnection>,
    mut cache: CacheImpl,
) -> Either<Redirect, CommonResponse> {
    let result = cache
        .get_by_hash(hash)
        .await
        .ok_or(Error::Internal("".to_string()))
        .or(UrlMapping::get_by_hash(hash, db.inner())
            .await
            .inspect(|model| {
                let model = model.clone();
                tokio::spawn(async move {
                    cache.write(&model.hash, &model.dest).await;
                });
            })
            .map_err(Error::from)
            .map(|m| m.dest));

    match result {
        Ok(dest) => Either::Left(Redirect::permanent(dest)),
        Err(err) => Either::Right(CommonResponse::from(err)),
    }
}

#[allow(unused)]
#[delete("/<hash>", rank = 1)]
#[instrument]
pub async fn delete_link(hash: &str) {
    todo!()
}

#[allow(unused)]
#[get("/<hash>/info")]
#[instrument]
pub async fn get_link_status(hash: &str) {
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

async fn get_or_create_url<C>(c: CreateRequest, db: &C, mut cache: CacheImpl) -> CommonResponse
where
    C: ConnectionTrait,
{
    match get_or_create(c, db).await.inspect(|model| {
        let model = model.clone();
        tokio::spawn(async move {
            cache.write(&model.hash, &model.dest).await;
        });
    }) {
        Ok(model) => CommonResponse::new(Status::Ok.code)
            .append("hash", serde_json::Value::String(model.hash)),
        Err(err) => err.into(),
    }
}
