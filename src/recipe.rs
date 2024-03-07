use std::collections::BTreeMap;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

//
fn fix_string(str: &str) -> String {
    // Remove hidden characters
    let re = Regex::new(r#"[\u0000-\u001F]"#).unwrap();
    let mut ret = Regex::replace_all(&re, &str, "").to_string();

    // Bring back nordic letters
    let special_characters = BTreeMap::from([
        ("&amp;Aring;", "Å"),
        ("&amp;Auml;", "Ä"),
        ("&amp;Ouml;", "Ö"),
        ("&amp;aring;", "å"),
        ("&amp;auml;", "ä"),
        ("&amp;ouml;", "ö"),
        ("&amp;Egrave;", "É"),
        ("&amp;egrave;", "é"),
        ("&amp;nbsp;", " "),
    ]);
    for (special_character, real_character) in special_characters.iter() {
        ret = ret.replace(special_character, real_character).to_string();
    }

    ret.to_string()
}

impl Recipe {
    pub fn from_url(url: &str) -> Recipe {
        let body = match reqwest::blocking::get(url) {
            Ok(resp) => resp.text().unwrap(),
            Err(e) => panic!("{e}"),
        };

        let document = scraper::Html::parse_document(&body);
        let json_selector =
            scraper::Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
        let mut data: Value = Value::Null;

        // Regex to remove stupid characters

        for tag in document.select(&json_selector) {
            let text = tag.inner_html();
            let text_filtered = fix_string(&text);
            let raw_map: Value = serde_json::from_str(&text_filtered).unwrap();
            if raw_map.is_array() {
                if let Some(i) = raw_map
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|element| element["@type"] == "Recipe")
                {
                    data = i.clone();
                    break;
                }
            } else if raw_map["@type"] == "Recipe" {
                data = raw_map;
                break;
            };
        }

        let mut recipe: Recipe = match serde_json::from_value(data) {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        };

        if recipe.url.is_none() {
            recipe.url = Some(url.to_string());
        }

        println!("{recipe:#?}");
        recipe
    }
}

pub struct Recipe2 {
    pub description: Option<String>,
    pub cooking_method: Option<String>,
    pub cook_time: Option<String>,
    pub date_modified: Option<Date>,
    pub date_published: Option<Date>,
    pub image: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub name: Option<String>,
    pub recipe_category: Option<Vec<String>>,
    pub recipe_cuisine: Option<Vec<String>>,
    pub recipe_ingredient: Option<Vec<String>>,
    pub recipe_instructions: Option<Vec<String>>,
    pub recipe_yield: Option<String>,
    pub total_time: Option<String>,
    pub url: Option<String>,
}

impl Recipe2 {
    pub fn from_recipe(recipe: Recipe) -> Recipe2 {
        Self {
            description: None,
            cooking_method: None,
            cook_time: None,
            date_modified: None,
            date_published: None,
            image: None,
            keywords: None,
            name: None,
            recipe_category: None,
            recipe_cuisine: None,
            recipe_ingredient: None,
            recipe_instructions: None,
            recipe_yield: None,
            total_time: None,
            url: None,
        }
    }
}
