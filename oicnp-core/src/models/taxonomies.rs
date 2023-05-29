use sea_orm::FromQueryResult;

use crate::{
    entities::{
        cms_taxonomies,
    },
};

#[derive(Clone, Debug, FromQueryResult)]
pub struct Taxonomies {
    pub tid: String,
    pub vid: String,
    pub pid: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}

impl Taxonomies {
    pub fn from_model(model: &cms_taxonomies::Model) -> Self {
        Self {
            tid: String::from(model.clone().tid),
            vid: String::from(model.clone().vid.unwrap_or("".to_string())),
            pid: String::from(model.clone().pid.unwrap_or("".to_string())),
            name: String::from(model.clone().name.unwrap_or("".to_string())),
            description: String::from(model.clone().description.unwrap_or("".to_string())),
            description_format: String::from(model.clone().description_format.unwrap_or("".to_string())),
            weight: model.weight.unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewTaxonomy {
    pub vid: String,
    pub pid: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}