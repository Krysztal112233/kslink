use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use moka::future::Cache;
use once_cell::sync::Lazy;
use rocket::async_trait;

use crate::cache::KVCache;

pub struct MokaCache(moka::future::Cache<String, String>);

static MOKA: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .time_to_idle(Duration::from_secs(5))
        .build()
});

impl MokaCache {
    pub fn new() -> Self {
        Self((*MOKA).clone())
    }
}

#[async_trait]
impl KVCache for MokaCache {
    async fn put(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string()).await;
    }

    async fn get(&mut self, key: &str) -> Option<String> {
        self.0.get(&key.to_string()).await
    }
}

impl Deref for MokaCache {
    type Target = moka::future::Cache<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MokaCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
