use deadpool_redis::{redis::AsyncTypedCommands, Connection};
use kslink_config::CacheConfig;
use rocket::async_trait;

use crate::cache::{moka::MokaCache, KVCache};

pub struct RedisCache {
    conn: Connection,
    config: CacheConfig,

    moka: MokaCache,
}

#[allow(unused)]
impl RedisCache {
    pub fn new(conn: Connection, config: CacheConfig, moka: MokaCache) -> Self {
        Self { conn, config, moka }
    }
}

#[async_trait]
impl KVCache for RedisCache {
    async fn put(&mut self, key: &str, value: &str) {
        self.moka.put(key, value).await;

        let _ = self.conn.set_ex(key, value, self.config.expire).await;
    }

    async fn get(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.moka.get(key).await {
            return Some(value);
        }

        self.conn
            .get_ex(
                key.to_string(),
                deadpool_redis::redis::Expiry::EX(self.config.expire),
            )
            .await
            .ok()
            .flatten()
    }
}
