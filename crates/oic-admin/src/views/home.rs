use askama::Template;
use crate::models::{AssetFiles, RenderBytes};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub ctx: WebAppContext,
    pub assets: AssetFiles,
    pub is_login_page: bool,
}

pub async fn render_home_index(ctx: &WebAppContext) -> Result<Bytes> {
    let assets = AssetFiles::default();
    
    let template = HomeTemplate {
        ctx: ctx.clone(),
        assets,
        is_login_page: false,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}

pub async fn render_auth_login(ctx: &WebAppContext) -> Result<Bytes> {
    let assets = AssetFiles::default();
    
    let template = HomeTemplate {
        ctx: ctx.clone(),
        assets,
        is_login_page: true,
    };
    
    template.render_bytes()
}