use educe::Educe;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KSLinkConfig {
    #[serde(default)]
    pub database: DatabaseConfig,

    #[serde(default)]
    pub redis: RedisConfig,

    #[serde(default, flatten)]
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe)]
#[educe(Default)]
pub struct DatabaseConfig {
    #[educe(Default = "postgres://postgres:postgres@postgres/postgres")]
    pub url: String,

    #[educe(Default = 16)]
    pub connect_timeout: u64,

    #[educe(Default = 64)]
    pub max_connections: u32,

    #[educe(Default = 8)]
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe)]
#[educe(Default)]
pub struct RedisConfig {
    #[educe(Default = "redis://redis:6379")]
    pub url: String,

    #[serde(flatten)]
    pub deadpool: deadpool_redis::PoolConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Educe)]
#[educe(Default)]
pub struct CacheConfig {
    #[educe(Default = 60)]
    pub expire: u64,
}

impl KSLinkConfig {
    pub fn get_figment() -> Figment {
        Figment::from(rocket::Config::default())
            .merge(Serialized::defaults(KSLinkConfig::default()))
            .merge(Toml::file("./kslink.toml").nested())
            .merge(Env::prefixed("KSLINK_").global())
            .select(Profile::from_env_or("KSLINK_PROFILE", "dev"))
    }
}
