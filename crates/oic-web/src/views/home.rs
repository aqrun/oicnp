use askama::Template;
use crate::models::ViteAssets;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;
use crate::models::HtmlTemplate;
use anyhow::Result;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub name: String,
    pub assets: ViteAssets,
}

pub async fn render_home_index(
    manifest: Arc<HashMap<String, ManifestChunk>>
) -> Result<HtmlTemplate<HomeTemplate>> {
    let m: HashMap<String, ManifestChunk> = (*manifest).clone();
    let assets: ViteAssets = ManifestChunk::get_assets_by_name(m, "main");
    let template = HomeTemplate {
        name: "World".to_string(),
        assets,
    };
    Ok(HtmlTemplate(template))
}