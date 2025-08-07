use im::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RuleMeta {
    pub r#type: RuleType,
    pub content: HashMap<String, RuleContent>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RuleContent {
    pub explain: Option<String>,
    pub param: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    #[default]
    Trim,
    Expand,
}
