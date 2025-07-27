use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use deadpool::managed::Pool;
use deadpool_redis::{
    redis::{aio::MultiplexedConnection, AsyncTypedCommands, Expiry, ToRedisArgs},
    Connection, Manager,
};
use kslink_config::{CacheConfig, KSLinkConfig};
use rocket::{
    async_trait,
    request::{FromRequest, Outcome},
    Request,
};

pub mod hasher;
pub mod request;
pub mod response;

pub type RedisPool = Pool<Manager, Connection>;

pub struct CacheLayer {
    conn: MultiplexedConnection,
    config: CacheConfig,
}

#[allow(unused)]
impl CacheLayer {
    pub fn new(conn: MultiplexedConnection, config: CacheConfig) -> Self {
        Self { conn, config }
    }

    pub async fn put<'a, K, V>(&mut self, key: K, value: V)
    where
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
    {
        let _ = self.conn.set_ex(key, value, self.config.expire).await;
    }

    pub async fn get<'a, K>(&mut self, key: K) -> Option<String>
    where
        K: ToRedisArgs + Send + Sync + 'a,
    {
        self.conn
            .get_ex(key, deadpool_redis::redis::Expiry::EX(self.config.expire))
            .await
            .ok()
            .flatten()
    }

    pub async fn get_by_hash<K>(&mut self, hash: K) -> Option<String>
    where
        K: Display,
    {
        self.conn
            .get_ex(
                format!("kslink.hash.{hash}"),
                Expiry::EX(self.config.expire),
            )
            .await
            .ok()?
    }

    pub async fn write<'a, K, V>(&mut self, hash: K, url: V)
    where
        K: Display,
        V: ToRedisArgs + Send + Sync + 'a,
    {
        self.conn
            .set_ex(format!("kslink.hash.{hash}"), url, self.config.expire)
            .await;
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for CacheLayer {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Infallible> {
        let config = request
            .rocket()
            .state::<KSLinkConfig>()
            .unwrap()
            .cache
            .clone();
        let conn = request
            .rocket()
            .state::<RedisPool>()
            .unwrap()
            .get()
            .await
            .unwrap()
            .clone();

        Outcome::Success(Self { config, conn })
    }
}

impl Debug for CacheLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheLayer")
            .field("config", &self.config)
            .finish()
    }
}
