use educe::Educe;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSLinkConfig {
    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe, PartialEq, Eq, PartialOrd, Ord)]
#[educe(Default)]
pub struct DatabaseConfig {
    #[educe(Default = "postgres://root:root@localhost/postgres")]
    pub url: String,

    #[educe(Default = 16)]
    pub connect_timeout: u64,

    #[educe(Default = 64)]
    pub max_connections: u32,

    #[educe(Default = 8)]
    pub min_connections: u32,
}

impl KSLinkConfig {
    pub fn new() -> Self {
        let figment = Figment::from(rocket::Config::default())
            .merge(Serialized::defaults(KSLinkConfig::default()))
            .merge(Serialized::defaults(rocket::Config::default()))
            .merge(Toml::file("/etc/kslink.toml").nested())
            .merge(Toml::file("./kslink.toml").nested())
            .merge(Env::prefixed("KSLINK_").split("_"))
            .select(Profile::from_env_or("KSLINK_PROFILE", "dev"));

        figment.extract().unwrap()
    }
}
