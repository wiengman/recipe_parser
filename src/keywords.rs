use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keywords {
    String(String),
    VecString(Vec<String>),
}
