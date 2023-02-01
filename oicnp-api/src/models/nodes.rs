use async_graphql::{Object, Context};
use crate::models::{Taxonomies, Users};
use crate::services::{
    find_user_by_id,
    find_node_body,
    find_node_taxonomies,
};
use crate::typings::{DateFormat, NodeBundle, TaxonomyBundle};
use oicnp_core::{DateTime, DatabaseConnection};
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Nodes {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: bool,
    pub created_at: DateTime,
    pub created_by: i32,
    pub updated_at: DateTime,
    pub updated_by: i32,
}

#[Object]
impl Nodes {
    async fn nid(&self) -> i32 {
        self.nid
    }
    async fn vid(&self) -> &str {
        self.vid.as_str()
    }
    async fn uid(&self) -> i32 {
        self.uid
    }
    async fn author(
        &self,
        ctx: &Context<'_>
    ) -> Option<Users> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(user) = find_user_by_id(rb, self.uid).await {
        //     return Some(user);
        // }
        None
    }
    async fn bundle(&self) -> &str {
        self.bundle.as_str()
    }
    async fn title(&self) -> &str {
        self.title.as_str()
    }
    async fn viewed(&self) -> i32 {
        self.viewed
    }
    async fn deleted(&self) -> bool {
        self.deleted
    }
    async fn created_by(
        &self,
        ctx: &Context<'_>
    ) -> Option<Users> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(user) = find_user_by_id(rb, self.created_by).await {
        //     return Some(user);
        // }
        None
    }
    async fn updated_by(
        &self,
        ctx: &Context<'_>
    ) -> Option<Users> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(user) = find_user_by_id(rb, self.updated_by).await {
        //     return Some(user);
        // }
        None
    }

    async fn created_at(&self) -> String {
        self.created_at.format(&DateFormat::Normal.to_string()).to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.format(&DateFormat::Normal.to_string()).to_string()
    }
    async fn category(
        &self,
        ctx: &Context<'_>
    ) -> Result<Taxonomies> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(res) = find_node_taxonomies(
        //     rb,
        //     &TaxonomyBundle::Category.to_string(),
        //     &self.nid
        // ).await {
        //     if let Some(res) = res.get(0) {
        //         return Ok(res.clone());
        //     }
        // }
        Err(anyhow!("Category not exist"))
    }
    async fn node_body(
        &self,
        ctx: &Context<'_>
    ) -> Result<NodeBody> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(res) = find_node_body(rb.clone(), self.nid).await {
        //     return Ok(res);
        // }
        Err(anyhow!("Node body not exist"))
    }
    async fn tags(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<Taxonomies>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // if let Ok(res) = find_node_taxonomies(
        //     rb,
        //     &TaxonomyBundle::Tag.to_string(),
        //     &self.nid
        // ).await {
        //     return Ok(res);
        // }
        Err(anyhow!("Tags not exist"))
    }
}

#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: i32,
    pub updated_by: i32,
}

#[derive(Clone, Debug)]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl NodeBody {
    async fn nid(&self) -> i32 {
        self.nid
    }
    async fn summary(&self) -> &str {
        self.summary.as_str()
    }
    async fn body(&self) -> &str {
        self.body.as_str()
    }
    async fn body_format(&self) -> &str {
        self.body_format.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct NodeTaxonomiesMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[derive(Clone, Debug)]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i64,
}

#[derive(Clone, Debug)]
pub struct NodeImagesMap {
    pub bundle: String,
    pub nid: i32,
    pub fid: i32,
    pub weight: i32,
    pub alt: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
}
