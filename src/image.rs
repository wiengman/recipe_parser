use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Image {
    String(String),
    ImageObject(ImageObject),
    VecString(Vec<String>),
    VecImageObject(Vec<ImageObject>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageObject {
    #[serde(alias = "caption")]
    pub name: String,
    pub url: String,
}
