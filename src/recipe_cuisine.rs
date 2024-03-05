use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeCuisine {
    String(String),
    VecString(Vec<String>),
}
