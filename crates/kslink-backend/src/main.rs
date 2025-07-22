#![feature(min_specialization)]

use std::io;

use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};
use kslink_config::KSLinkConfig;
use mimalloc::MiMalloc;

use crate::middleware::handler;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod common;
mod endpoints;
mod middleware;

#[actix_web::main]
async fn main() -> io::Result<()> {
    simple_logger::init().unwrap();

    let config = KSLinkConfig::new();

    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::default().default_handler(handler::default_error_handler))
            .service(
                web::scope("/link")
                    .service(endpoints::link::post_with_json)
                    .service(endpoints::link::post_with_query)
                    .service(endpoints::link::get_link)
                    .service(endpoints::link::delete_link),
            )
            .service(web::scope("/statistics"))
            .service(web::scope("/management"))
    })
    .bind((config.actix.host, config.actix.port))?
    .workers(config.actix.workers)
    .run()
    .await
}
