use async_graphql::{Object, Context};
use crate::typings::{DateFormat, TaxonomyBundle};
use crate::services::{
    find_user_by_id,
    find_node_taxonomies,
};
use crate::models::{
    Nodes,
    NodeBody,
    Users,
    Taxonomies,
};
use oicnp_core::{
    DatabaseConnection,
    prelude::{
        chrono::prelude::*,
    },
    models::{DetailNode as CoreDetailNode},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailNode {
    pub nid: String,
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_by_username: String,
    pub updated_by_nickname: String,

    pub tid: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: String,
    pub author_username: String,
    pub author_nickname: String,

    pub summary: String,
    pub summary_format: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl DetailNode {
    async fn nid(&self) -> &str {
        self.nid.as_str()
    }
    async fn vid(&self) -> &str {
        self.vid.as_str()
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

    async fn created_at(&self) -> String {
        self.created_at.format(&DateFormat::Normal.to_string()).to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.format(&DateFormat::Normal.to_string()).to_string()
    }

    async fn created_by(&self) -> &str {
        self.created_by.as_str()
    }
    async fn updated_by(&self) -> &str {
        self.updated_by.as_str()
    }
    async fn updated_by_username(&self) -> &str {
        self.updated_by_username.as_str()
    }
    async fn updated_by_nickname(&self) -> &str {
        self.updated_by_nickname.as_str()
    }

    async fn tid(&self) -> &str {
        self.tid.as_str()
    }
    async fn category_name(&self) -> &str {
        self.category_name.as_str()
    }
    async fn category_vid(&self) -> &str {
        self.category_vid.as_str()
    }

    async fn author_uid(&self) -> &str {
        self.author_uid.as_str()
    }
    async fn author_username(&self) -> &str {
        self.author_username.as_str()
    }
    async fn author_nickname(&self) -> &str {
        self.author_nickname.as_str()
    }

    async fn summary(&self) -> &str {
        self.summary.as_str()
    }
    async fn summary_format(&self) -> &str {
        self.summary_format.as_str()
    }
    async fn body(&self) -> &str {
        self.body.as_str()
    }
    async fn body_format(&self) -> &str {
        self.body_format.as_str()
    }
}

impl From<&CoreDetailNode> for DetailNode {
    fn from(n: &CoreDetailNode) -> Self {
        Self {
            nid: String::from(n.nid.as_str()),
            vid: String::from(n.vid.as_str()),
            bundle: String::from(n.bundle.as_str()),
            title: String::from(n.title.as_str()),
            viewed: n.viewed,
            deleted: n.deleted.eq("1"),
            created_at: n.created_at,
            created_by: String::from(n.created_by.as_str()),
            updated_at: n.updated_at,
            updated_by: String::from(n.updated_by.as_str()),
            updated_by_username: String::from(n.updated_by_username.as_str()),
            updated_by_nickname: String::from(n.updated_by_nickname.as_str()),
            tid: String::from(n.tid.as_str()),
            category_name: String::from(n.category_name.as_str()),
            category_vid: String::from(n.category_vid.as_str()),
            author_uid: String::from(n.author_uid.as_str()),
            author_username: String::from(n.author_username.as_str()),
            author_nickname: String::from(n.author_nickname.as_str()),
            summary: String::from(n.summary.as_str()),
            summary_format: String::from(n.summary_format.as_str()),
            body: String::from(n.body.as_str()),
            body_format:String::from(n.body_format.as_str()),
        }
    }
}