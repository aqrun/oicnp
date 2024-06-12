use crate::{
    entities::prelude::*,
    DateTime,
    typings::DateFormat,
    DbConn,
    services::find_node_tags,
};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use async_graphql::{Object, SimpleObject, Context};

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct Node {
    pub nid: i64,
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    pub created_at: DateTime,
    pub created_by: i64,
    pub updated_at: DateTime,
    pub updated_by: i64,
    pub updated_by_username: Option<String>,
    pub updated_by_nickname: Option<String>,

    pub tid: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: Option<String>,
    pub author_username: Option<String>,
    pub author_nickname: Option<String>,

    pub summary: String,
    pub summary_format: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl Node {
    async fn nid(&self) -> i64 {
        self.nid
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
        self.deleted.eq("1")
    }
    async fn created_at(&self) -> String {
        self.created_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }
    async fn created_by(&self) -> i64 {
        self.created_by
    }
    async fn updated_by(&self) -> i64 {
        self.updated_by
    }
    async fn updated_by_username(&self) -> String {
        self.clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn updated_by_nickname(&self) -> String {
        self.clone()
            .updated_by_username
            .unwrap_or("".to_string())
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

    async fn author_uid(&self) -> String {
        self.clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn author_username(&self) -> String {
        self.clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn author_nickname(&self) -> String {
        self.clone()
            .updated_by_username
            .unwrap_or("".to_string())
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

    async fn tags(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<TagModel> {
        let db = ctx.data_unchecked::<DbConn>();
        let res = find_node_tags(
            db,
            self.nid
        ).await;

        if let Ok(res) = res {
            return res;
        }

        return Vec::new();
    }
}

#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: i64,
    pub updated_by: i64,
}

#[derive(Clone, Debug, FromQueryResult)]
pub struct NodeCount {
    pub count: i32,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct ResNodeList {
    pub data: Vec<Node>,
    pub page_info: crate::typings::PagerInfo,
}
