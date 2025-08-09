use im::HashMap;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RuleMeta {
    pub r#type: RuleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<HashMap<Url, Url>>,
    pub content: HashMap<String, RuleContent>,
}

impl RuleMeta {
    pub fn merge(self, meta: Self) -> Self {
        Self {
            content: self.content.union(meta.content),
            ..self
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RuleContent {
    pub explain: Option<String>,
    pub queries: HashMap<String, Option<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    #[default]
    Trim,
    Expand,
}
