use loco_rs::prelude::*;
use serde_json::json;

/// admin view
///
/// # Errors
///
/// This function will return an error if render fails
pub fn index(v: &impl ViewRenderer) -> Result<Response> {
    let assets_base_url = "http://static.oicnp.com/";
    let assets_app = "admin";
    let assets_version = "1.0.0";
    let style_files = vec![
        "index.css",
    ].into_iter()
    .map(|item| {
        format!("{assets_base_url}{assets_app}/{assets_version}/{item}")
    })
    .collect::<Vec<String>>();
    let js_files = vec![
        "index.js"
    ].into_iter()
    .map(|item| {
        format!("{assets_base_url}{assets_app}/{assets_version}/{item}")
    })
    .collect::<Vec<String>>();

    format::render().view(v, "admin/index.html", json!({
        "style_files": style_files,
        "js_files": js_files
    }))
}
