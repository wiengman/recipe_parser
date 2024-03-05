use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeYield {
    VecString(Vec<String>),
    String(String),
    Integer(i32),
}
