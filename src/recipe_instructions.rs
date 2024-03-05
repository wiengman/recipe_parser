use serde::{Deserialize, Serialize};

use crate::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeInstructions {
    String(String),
    VecString(Vec<String>),
    VecHowToStep(Vec<HowToStep>),
    VecHowToSection(Vec<HowToSection>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HowToStep {
    pub text: String,
    pub image: Option<Image>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HowToSection {
    pub name: Option<String>,
    pub item_list_element: Vec<HowToStep>,
}
