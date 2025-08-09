use std::{fmt::Debug, hash::Hash};

use im::{HashMap, HashSet};
use log::error;
use rayon::prelude::*;
use url::Url;

use crate::{meta::RuleMeta, PrunedUrl, RuleStore, WrappedRegex};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SimpleRuleStore {
    explain: Option<String>,
    queries: HashMap<WrappedRegex, Option<String>>,
}

impl SimpleRuleStore {
    pub fn new(meta: RuleMeta) -> HashMap<WrappedRegex, Self> {
        meta.content
            .iter()
            .map(|(reg, content)| {
                (
                    WrappedRegex::new(reg)
                        .inspect_err(|err| error!("while try compile {reg}: {err}")),
                    content,
                )
            })
            .filter(|(reg, _)| reg.is_ok())
            .map(|(reg, content)| (reg.unwrap(), content))
            .map(|(reg, content)| {
                let queries = content
                    .queries
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
                (reg, Self { explain, queries })
            })
            .collect::<HashMap<WrappedRegex, SimpleRuleStore>>()
    }
}

#[async_trait::async_trait]
impl RuleStore for SimpleRuleStore {
    async fn run(&self, url: &Url) -> PrunedUrl {
        let banned = url
            .query_pairs()
            .filter_map(|(k, _)| {
                let key = k.to_string();
                if self.queries.keys().par_bridge().any(|re| re.is_match(&key)) {
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

        banned
            .iter()
            .map(|query| (url.query_pairs().find(|(e, _)| e == query).unwrap().clone()))
            .fold(PrunedUrl::new(new_url.clone()), |acc, (k, v)| {
                acc.append(k.clone().to_string(), v.clone().to_string())
            })
    }
}
