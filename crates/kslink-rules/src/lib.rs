use std::{fmt::Debug, hash::Hash, sync::Arc};

use im::{HashMap, HashSet};
use log::error;
use rayon::prelude::*;
use regex::Regex;
use url::Url;

use crate::error::Result;
use crate::meta::RuleMeta;

pub mod error;
pub mod meta;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct SimpleRuleStore {
    explain: Option<String>,
    param: HashMap<WrappedRegex, String>,
}

impl SimpleRuleStore {
    pub fn new(meta: RuleMeta) -> HashMap<WrappedRegex, Self> {
        meta.content
            .iter()
            .map(|(reg, rule)| {
                (
                    WrappedRegex::new(reg)
                        .inspect_err(|err| error!("while try compile {reg}: {err}")),
                    rule,
                )
            })
            .filter(|(reg, _)| reg.is_ok())
            .map(|(reg, content)| (reg.unwrap(), content))
            .map(|(reg, content)| {
                let param = content
                    .param
                    .iter()
                    .map(|(reg, explain)| {
                        (
                            WrappedRegex::new(reg)
                                .inspect_err(|err| error!("while try compile {reg}: {err}")),
                            explain.to_owned(),
                        )
                    })
                    .filter(|(rex, _)| rex.is_ok())
                    .map(|(rex, explain)| (rex.unwrap(), explain))
                    .collect::<HashMap<_, _>>();
                let explain = content.explain.clone();
                (reg, Self { explain, param })
            })
            .collect::<HashMap<WrappedRegex, SimpleRuleStore>>()
    }
}

#[async_trait::async_trait]
impl RuleStore for SimpleRuleStore {
    async fn run(&self, url: &Url) -> Url {
        let banned = url
            .query_pairs()
            .filter_map(|(k, _)| {
                let key = k.to_string();
                if self.param.keys().par_bridge().any(|re| re.is_match(&key)) {
                    Some(key)
                } else {
                    None
                }
            })
            .collect::<HashSet<String>>();

        let mut new_url = url.clone();
        let kept: Vec<_> = new_url
            .query_pairs()
            .filter(|(k, _)| !banned.contains(&k.to_string()))
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        new_url.query_pairs_mut().clear().extend_pairs(&kept);

        new_url
    }
}
