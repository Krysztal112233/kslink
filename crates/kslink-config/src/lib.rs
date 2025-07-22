use educe::Educe;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSLinkConfig {
    #[serde(default)]
    pub actix: ActixConfig,

    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe, PartialEq, Eq, PartialOrd, Ord)]
#[educe(Default)]
pub struct ActixConfig {
    #[educe(Default = 4)]
    #[serde(default)]
    pub workers: usize,

    #[educe(Default = "0.0.0.0")]
    #[serde(default)]
    pub host: String,

    #[educe(Default = 9000)]
    #[serde(default)]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe, PartialEq, Eq, PartialOrd, Ord)]
#[educe(Default)]
pub struct DatabaseConfig {
    #[educe(Default = "postgres://root:root@localhost/postgres")]
    pub url: String,
}

impl KSLinkConfig {
    pub fn new() -> Self {
        let figment = Figment::new()
            .merge(Serialized::defaults(KSLinkConfig::default()))
            .merge(Env::prefixed("KSLINK_"))
            .merge(Toml::file("/etc/kslink.toml"))
            .merge(Toml::file("./kslink.toml"))
            .select(Profile::from_env_or("KSLINK_PROFILE", "dev"));

        figment.extract().unwrap()
    }
}
