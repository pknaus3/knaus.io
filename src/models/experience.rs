use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub period: String,
    pub location: String,
    pub description: String,
}


pub fn load_experience() -> Vec<Experience> {
    let experience: String = fs::read_to_string("src/data/experiences.json").expect("Error loading experience");

    let data: Vec<Experience> = serde_json::from_str(&experience).expect("Convert error");

    data
}
