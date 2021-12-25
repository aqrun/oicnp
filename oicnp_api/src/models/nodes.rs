use async_graphql::{Object, Context};
use crate::models::{
    Users,
};
use crate::services::{
    find_user_by_id,
    find_node_body,
};
use crate::typings::{GqlState, DateFormat};

#[crud_table]
#[derive(Clone, Debug)]
pub struct Nodes {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: bool,
    pub created_at: rbatis::DateTimeNative,
    pub created_by: i32,
    pub updated_at: rbatis::DateTimeNative,
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
    async fn user(
        &self,
        ctx: &Context<'_>
    ) -> Option<Users> {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        if let Ok(user) = find_user_by_id(rb, self.uid).await {
            return Some(user);
        }
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
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        if let Ok(user) = find_user_by_id(rb, self.created_by).await {
            return Some(user);
        }
        None
    }
    async fn updated_by(
        &self,
        ctx: &Context<'_>
    ) -> Option<Users> {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        if let Ok(user) = find_user_by_id(rb, self.updated_by).await {
            return Some(user);
        }
        None
    }

    async fn created_at(&self) -> String {
        self.created_at.format(&DateFormat::Normal.to_string()).to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.format(&DateFormat::Normal.to_string()).to_string()
    }
}

#[crud_table(table_name: nodes)]
#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_by: i32,
    pub updated_by: i32,
}

#[crud_table(table_name: node_body)]
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

#[crud_table(table_name: node_taxonomies_map)]
#[derive(Clone, Debug)]
pub struct NodeTaxonomiesMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[crud_table(table_name: node_comments_map)]
#[derive(Clone, Debug)]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i64,
}

#[crud_table(table_name: node_images_map)]
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
