use std::{fmt::Debug, hash::Hash, sync::Arc};

use im::HashMap;
use regex::Regex;
use url::Url;

use crate::{error::Result, meta::RuleMeta, store::SimpleRuleStore};

pub mod error;
pub mod meta;
mod store;

#[derive(Debug, Clone)]
struct WrappedRegex(Regex);

impl std::ops::Deref for WrappedRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WrappedRegex {
    pub fn new(regex: &str) -> Result<WrappedRegex> {
        Ok(WrappedRegex(Regex::new(regex)?))
    }
}

impl Hash for WrappedRegex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl Eq for WrappedRegex {}

impl PartialEq for WrappedRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

#[derive(Default)]
pub struct RuleSet {
    store: HashMap<WrappedRegex, Arc<Box<dyn RuleStore>>>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn join(self, meta: RuleMeta) -> Self {
        let f = |s: SimpleRuleStore| -> Arc<Box<dyn RuleStore>> { Arc::new(Box::new(s)) };

        Self {
            store: self.store.union(
                SimpleRuleStore::new(meta)
                    .into_iter()
                    .map(|(w, s)| (w, f(s)))
                    .collect(),
            ),
        }
    }
}

#[allow(clippy::mutable_key_type)]
#[async_trait::async_trait]
pub trait RuleStore: Sync + Send + Debug {
    async fn run(&self, url: &Url) -> Url;
}
