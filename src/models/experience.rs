use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub period: String,
    pub location: String,
    pub description: String,
}
