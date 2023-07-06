use crate::typings::DateFormat;
use async_graphql::Object;
use oicnp_core::models::DetailNode as CoreDetailNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailNode {
    pub data: CoreDetailNode,
}

#[Object]
impl DetailNode {
    async fn nid(&self) -> &str {
        self.data.nid.as_str()
    }
    async fn vid(&self) -> &str {
        self.data.vid.as_str()
    }
    async fn bundle(&self) -> &str {
        self.data.bundle.as_str()
    }
    async fn title(&self) -> &str {
        self.data.title.as_str()
    }
    async fn viewed(&self) -> i32 {
        self.data.viewed
    }
    async fn deleted(&self) -> bool {
        self.data.deleted.eq("1")
    }
    async fn created_at(&self) -> String {
        self.data
            .created_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }
    async fn updated_at(&self) -> String {
        self.data
            .updated_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }
    async fn created_by(&self) -> &str {
        self.data.created_by.as_str()
    }
    async fn updated_by(&self) -> &str {
        self.data.updated_by.as_str()
    }
    async fn updated_by_username(&self) -> String {
        self.data
            .clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn updated_by_nickname(&self) -> String {
        self.data
            .clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn tid(&self) -> &str {
        self.data.tid.as_str()
    }
    async fn category_name(&self) -> &str {
        self.data.category_name.as_str()
    }
    async fn category_vid(&self) -> &str {
        self.data.category_vid.as_str()
    }

    async fn author_uid(&self) -> String {
        self.data
            .clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn author_username(&self) -> String {
        self.data
            .clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn author_nickname(&self) -> String {
        self.data
            .clone()
            .updated_by_username
            .unwrap_or("".to_string())
    }
    async fn summary(&self) -> &str {
        self.data.summary.as_str()
    }
    async fn summary_format(&self) -> &str {
        self.data.summary_format.as_str()
    }
    async fn body(&self) -> &str {
        self.data.body.as_str()
    }
    async fn body_format(&self) -> &str {
        self.data.body_format.as_str()
    }
}
