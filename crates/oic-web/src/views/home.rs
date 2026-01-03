use askama::Template;
use crate::models::ViteAssets;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub name: String,
    pub assets: ViteAssets,
}