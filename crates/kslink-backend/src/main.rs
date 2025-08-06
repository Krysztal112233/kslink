use std::time::Duration;

use deadpool::Runtime;
use kslink_config::{DatabaseConfig, KSLinkConfig, RedisConfig};
use mimalloc::MiMalloc;
use rocket::{
    catchers,
    fairing::AdHoc,
    launch, routes,
    tokio::{self, sync::mpsc},
    Rocket,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use snowflake_ng::SnowflakeGenerator;
use tracing::level_filters::LevelFilter;

use crate::{
    cache::RedisPool,
    common::fairing::{Cors, VisitQueueConsumer},
    endpoints::{root, statistics},
    error::Error,
    middleware::handler,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod cache;
mod common;
mod endpoints;
mod error;
mod middleware;

#[launch]
async fn rocket() -> _ {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let config = KSLinkConfig::get_figment();
    let kslink_config: KSLinkConfig = config.extract().unwrap();
    let database = setup_database(&kslink_config.database).await.unwrap();
    let redis = setup_redis(&kslink_config.redis).await.unwrap();

    let ((tx, rx), c_db) = (mpsc::unbounded_channel(), database.clone());

    Rocket::custom(config)
        .register("/", catchers![handler::default])
        .attach(Cors)
        .attach(AdHoc::on_liftoff("Visit Record Consumer", |r| {
            Box::pin(async move {
                tokio::spawn(VisitQueueConsumer::run(c_db, rx, r.shutdown()));
            })
        }))
        .manage(database)
        .manage(redis)
        .manage(kslink_config)
        .manage(tx)
        .manage(SnowflakeGenerator::default())
        .mount(
            "/",
            routes![
                root::all_options,
                root::get_link,
                root::post_with_json,
                root::post_with_query,
            ],
        )
        .mount("/statistics", routes![statistics::get_statistics])
}

async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseConnection, Error> {
    let mut opt = ConnectOptions::new(config.url.to_owned());
    opt.min_connections(config.min_connections)
        .max_connections(config.max_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout));

    Ok(Database::connect(opt).await?)
}

async fn setup_redis(config: &RedisConfig) -> anyhow::Result<RedisPool> {
    let mut pool = deadpool_redis::Config::from_url(config.url.clone());

    pool.pool = Some(config.deadpool);

    Ok(pool.create_pool(Some(Runtime::Tokio1))?)
}
