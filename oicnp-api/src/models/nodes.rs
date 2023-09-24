use async_graphql::{Object, Context};
use crate::models::{Taxonomies, Users};
use crate::services::{
    find_node_body,
    find_node_taxonomies,
};
use crate::typings::{DateFormat, TaxonomyBundle};
use oicnp_core::{
    DateTime, DatabaseConnection,
    entities::{
        cms_nodes,
    },
    prelude::{
        chrono::prelude::*,
        anyhow::{anyhow, Result}
    },
    models::{Node, NodeBody as CoreNodeBody},
    typings::NodeBundle,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Nodes {
    pub nid: String,
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    pub created_at: DateTime,
    pub created_by: String,
    pub updated_at: DateTime,
    pub updated_by: String,
}

#[Object]
impl Nodes {
    async fn nid(&self) -> &str {
        self.nid.as_str()
    }
    async fn vid(&self) -> &str {
        self.vid.as_str()
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
    async fn deleted(&self) -> &str {
        self.deleted.as_str()
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
        let res = find_node_taxonomies(db, &self.nid).await?;

        if let Some(res) = res.get(0) {
            return Ok(res.clone());
        }

        Err(anyhow!("Category not exist"))
    }
    async fn node_body(
        &self,
        ctx: &Context<'_>
    ) -> Result<NodeBody> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = find_node_body(db, &self.nid).await?;
        Ok(res)
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
        Ok(Vec::new())
    }
}

impl From<&Node> for Nodes {
    fn from(node: &Node) -> Self {
        Self {
            nid: String::from(&node.nid),
            vid: String::from(&node.vid),
            bundle: String::from(&node.nid),
            title: String::from(&node.nid),
            viewed: node.viewed,
            deleted: String::from(&node.deleted),
            created_at: Default::default(),
            created_by: String::from(""),
            updated_at: Default::default(),
            updated_by: String::from(""),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub uid: String,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug)]
pub struct NodeBody {
    pub nid: String,
    pub summary: String,
    pub summary_format: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl NodeBody {
    async fn nid(&self) -> &str {
        self.nid.as_str()
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

impl From<&CoreNodeBody> for NodeBody {
    fn from(nb: &CoreNodeBody) -> Self {
        Self {
            nid: String::from(&nb.nid),
            summary: String::from(&nb.summary),
            summary_format: String::from(&nb.summary_format),
            body: String::from(&nb.body),
            body_format: String::from(&nb.body_format),
        }
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
