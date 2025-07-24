use std::time::Duration;

use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};
use kslink_config::{DatabaseConfig, KSLinkConfig};
use mimalloc::MiMalloc;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{error::Error, middleware::handler};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod common;
mod endpoints;
mod error;
mod middleware;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let config = KSLinkConfig::new();
    let database = setup_database(&config.database).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::default().default_handler(handler::default_error_handler))
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/link")
                    .service(endpoints::link::post_with_json)
                    .service(endpoints::link::post_with_query)
                    .service(endpoints::link::get_link)
                    .service(endpoints::link::get_link_status)
                    .service(endpoints::link::delete_link),
            )
            .service(web::scope("/statistics"))
            .service(web::scope("/management"))
    })
    .bind((config.actix.host, config.actix.port))?
    .workers(config.actix.workers)
    .run()
    .await?;

    Ok(())
}

async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseConnection, Error> {
    let mut opt = ConnectOptions::new(config.url.to_owned());
    opt.min_connections(config.min_connections)
        .max_connections(config.max_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout));

    Ok(Database::connect(opt).await?)
}
