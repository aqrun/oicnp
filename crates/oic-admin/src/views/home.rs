use askama::Template;
use crate::models::{AssetFiles, RenderBytes};
use crate::services::describe_node_list;
use crate::WebAppContext;
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::JsonResPayload,
};
use anyhow::Result;
use bytes::Bytes;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub ctx: WebAppContext,
    pub assets: AssetFiles,
}

pub async fn render_home_index(ctx: &WebAppContext) -> Result<Bytes> {
    let assets = AssetFiles::default();
    
    let template = HomeTemplate {
        ctx: ctx.clone(),
        assets,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}
