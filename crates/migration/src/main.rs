use std::env;

use kslink_config::KSLinkConfig;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    setup();

    cli::run_cli(migration::Migrator).await;
}

fn setup() {
    let config: KSLinkConfig = KSLinkConfig::get_figment().extract().unwrap();

    unsafe {
        env::set_var("DATABASE_URL", config.database.url);
    }
}
