use std::fs;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub category: String,
    pub items: Vec<String>,
}


pub fn load_skills() -> Vec<Skill> {
    let skills: String = fs::read_to_string("src/data/skills.json").expect("Error loading skills");

    let data: Vec<Skill> = serde_json::from_str(&skills).expect("Convert error");

    return data;
}
