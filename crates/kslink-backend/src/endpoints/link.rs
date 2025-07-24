use actix_web::{delete, get, post, web, HttpResponse};
use entity::model::url_mapping;
use sea_orm::ConnectionTrait;

use crate::{
    common::{request::CreateRequest, response::CommonResponse},
    error::Error,
};

#[post("/")]
async fn post_with_form(form: web::Form<CreateRequest>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

#[post("/")]
async fn post_with_json(form: web::Json<CreateRequest>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

#[post("/")]
async fn post_with_query(query: web::Query<CreateRequest>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

#[get("/{hash}")]
async fn get_link(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

#[delete("/{hash}")]
async fn delete_link(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

#[get("/{hash}/info")]
async fn get_link_status(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(CommonResponse::new(200))
}

async fn url_create_or_fetch<C>(c: CreateRequest, db: &C) -> Result<url_mapping::Model, Error>
where
    C: ConnectionTrait,
{
    todo!()
}
