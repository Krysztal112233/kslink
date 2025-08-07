use std::{fmt::Debug, hash::Hash};

use im::{HashMap, HashSet};
use log::error;
use rayon::prelude::*;
use url::Url;

use crate::{meta::RuleMeta, RuleStore, WrappedRegex};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SimpleRuleStore {
    explain: Option<String>,
    param: HashMap<WrappedRegex, String>,
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
