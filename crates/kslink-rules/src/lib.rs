use std::{fmt::Debug, hash::Hash, ops::Deref, sync::Arc};

use im::HashMap;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use url::Url;

use crate::{error::Result, meta::RuleMeta, store::SimpleRuleStore};

pub mod cli;
pub mod error;
pub mod meta;
pub(crate) mod store;

#[derive(Debug, Clone)]
struct WrappedRegex(Regex);

impl Deref for WrappedRegex {
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

#[derive(Debug, Default)]
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

    pub async fn prune(&self, url: &Url) -> Option<Url> {
        let (_, rule) = self
            .store
            .iter()
            .par_bridge()
            .find_first(|(e, _)| e.is_match(url.as_ref()))?;
        Some(rule.run(url).await)
    }
}

impl From<RuleMeta> for RuleSet {
    fn from(value: RuleMeta) -> Self {
        Self::default().join(value)
    }
}

#[allow(clippy::mutable_key_type)]
#[async_trait::async_trait]
pub trait RuleStore: Sync + Send + Debug {
    async fn run(&self, url: &Url) -> Url;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"
    type = "trim"

    tests = { "https://live.bilibili.com/?spm_id_from=333.1007.0.0" = "https://live.bilibili.com/?" }

    [content.'^https?://live\.bilibili\.com(?:/|$)']
    explain = "Matching the live site of bilibili"

    [content.'^https?://live\.bilibili\.com(?:/|$)'.param]
    launch_id = "Which method user enter this live"
    live_from = "Detail of method of the user enter this live"
    session_id = "Tracing"
    spm_id_from = '"Super Position Model" format'

    [content.'^https?://(?:www\.)?bilibili\.com(?:/|$)']
    explain = "Matching the main site of bilibili"

    [content.'^https?://(?:www\.)?bilibili\.com(?:/|$)'.param]
    spm_id_from = '"Super Position Model" format'
    vd_source = "Share tracing"
    "#;

    #[tokio::test]
    async fn test_simple() {
        let meta = toml::from_str::<RuleMeta>(TEST).unwrap();
        let rule = RuleSet::from(meta.clone());

        let tests = meta.tests.clone().unwrap();
        for (input, except) in tests.iter() {
            assert_eq!(rule.prune(input).await.unwrap(), *except);
        }
    }
}
