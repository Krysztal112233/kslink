use std::ops::{Deref, DerefMut};
use std::{convert::Infallible, fmt::Debug};

use deadpool::managed::Pool;
use deadpool_redis::{Connection, Manager};
use kslink_config::KSLinkConfig;
use rocket::{
    async_trait,
    request::{FromRequest, Outcome},
    Request,
};

use crate::cache::moka::MokaCache;
use crate::cache::redis::RedisCache;

pub mod moka;
pub mod redis;

pub type RedisPool = Pool<Manager, Connection>;

#[allow(unused)]
#[async_trait]
pub trait KVCache: Sync + Send {
    async fn put(&mut self, key: String, value: String);

    async fn get(&mut self, key: String) -> Option<String>;

    async fn get_by_hash(&mut self, hash: String) -> Option<String> {
        self.get(format!("kslink.hash.{hash}")).await
    }

    async fn write(&mut self, hash: String, url: String) {
        self.put(format!("kslink.hash.{hash}"), url.to_string())
            .await;
    }
}

pub struct CacheImpl(Box<dyn KVCache>);

impl Default for CacheImpl {
    fn default() -> Self {
        Self(Box::new(MokaCache::new()))
    }
}

impl From<MokaCache> for CacheImpl {
    fn from(value: MokaCache) -> Self {
        Self(Box::new(value))
    }
}

impl From<RedisCache> for CacheImpl {
    fn from(value: RedisCache) -> Self {
        Self(Box::new(value))
    }
}

impl Deref for CacheImpl {
    type Target = Box<dyn KVCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CacheImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for CacheImpl {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Infallible> {
        let config = request
            .rocket()
            .state::<KSLinkConfig>()
            .map(|it| it.cache.clone())
            .clone()
            .unwrap();

        let pool = request.rocket().state::<RedisPool>().unwrap();

        let cache = pool
            .get()
            .await
            .map(|conn| RedisCache::new(conn, config, MokaCache::new()))
            .map(CacheImpl::from)
            .unwrap_or_default();

        Outcome::Success(cache)
    }
}

impl Debug for CacheImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CacheLayer").field(&"##").finish()
    }
}

unsafe impl Send for CacheImpl {}
unsafe impl Sync for CacheImpl {}
