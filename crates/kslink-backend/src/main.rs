use std::time::Duration;

use kslink_config::{DatabaseConfig, KSLinkConfig};
use mimalloc::MiMalloc;
use rocket::{routes, Rocket};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::level_filters::LevelFilter;

use crate::{endpoints::root, error::Error};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod common;
mod endpoints;
mod error;
mod middleware;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let config = KSLinkConfig::new();
    let database = setup_database(&config.database).await?;

    Rocket::build()
        .manage(database)
        .mount(
            "/",
            routes![
                root::get_link,
                root::get_link_status,
                root::post_with_json,
                root::post_with_query,
                root::delete_link
            ],
        )
        .launch()
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
