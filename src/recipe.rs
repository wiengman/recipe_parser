use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

use crate::{
    date::Date, image::Image, keywords::Keywords, recipe_category::RecipeCategory,
    recipe_cuisine::RecipeCuisine, recipe_instructions::RecipeInstructions,
    recipe_yield::RecipeYield,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub description: Option<String>,
    pub cooking_method: Option<String>,
    pub cook_time: Option<String>,
    pub date_modified: Option<Date>,
    pub date_published: Option<Date>,
    pub image: Option<Image>,
    pub keywords: Option<Keywords>,
    pub name: Option<String>,
    pub recipe_category: Option<RecipeCategory>,
    pub recipe_cuisine: Option<RecipeCuisine>,
    pub recipe_ingredient: Option<Vec<String>>,
    pub recipe_instructions: Option<RecipeInstructions>,
    pub recipe_yield: Option<RecipeYield>,
    pub total_time: Option<String>,
    pub url: Option<String>,
}

impl Recipe {
    pub fn from_url(url: &str) -> Recipe{
        let body = match reqwest::blocking::get(url) {
            Ok(resp) => resp.text().unwrap(),
            Err(e) => panic!("{e}"),
        };

        let document = scraper::Html::parse_document(&body);
        let json_selector =
            scraper::Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
        let mut raw = String::new();

        // Regex to remove stupid characters
        
        let re = Regex::new(r#"[\u0000-\u001F]"#).unwrap();
        for tag in document.select(&json_selector) {
            let schema_raw = tag.inner_html();
            let schema_filtered = Regex::replace_all(&re, &schema_raw, "");
            let map: Value = serde_json::from_str(&schema_filtered).unwrap();
            if map.is_array() {
                if let Some(i) = map
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|element| element["@type"] == "Recipe")
                {
                    println!("{map:#?}");
                    raw = i.to_string();
                    break;
                }
            } else if map["@type"] == "Recipe" {
                println!("{map:#?}");
                raw = schema_filtered.to_string();
                break;
            };
        }
        
        match serde_json::from_str(&raw) {
            Ok(res) => res,
            Err(e) => panic!("{e}")
        }
    }
}
