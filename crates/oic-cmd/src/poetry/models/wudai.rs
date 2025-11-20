use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct WudaiPoetryModel {
    pub title: String,
    pub notes: Vec<String>,
    pub paragraphs: Vec<String>,
    pub author: String,
    pub rhythmic: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct AuthorModel {
    pub name: String,
    pub desc: String,
    pub description: String,
    #[serde(rename(deserialize = "shortDescription", serialize = "shortDescription"))]
    pub short_description: String,
}
