use async_graphql::{Object, Context};
use serde::{Deserialize, Serialize};
use crate::typings::{GqlState, DateFormat, TaxonomyBundle};
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
use rbatis::DateTimeNative;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailNode {
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
    pub created_by_username: String,
    pub created_by_nickname: String,
    pub updated_by_username: String,
    pub updated_by_nickname: String,

    pub tid: i32,
    pub category_bundle: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: i32,
    pub author_username: String,
    pub author_nickname: String,

    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl DetailNode {
    async fn nid(&self) -> i32 {
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
        self.deleted
    }

    async fn created_by(&self) -> Users {
        let a = self.clone();
        Users {
            uid: a.created_by,
            username: a.created_by_username,
            nickname: a.created_by_nickname,
            password: "".to_string(),
            status: 0,
            email: "".to_string(),
            admin: false,
            intro: "".to_string(),
            last_login_on: DateTimeNative::now(),
            salt: "".to_string(),
            must_change_password: false,
            password_changed_on: 0,
            created_at: DateTimeNative::now(),
            updated_at: DateTimeNative::now()
        }
    }

    async fn updated_by(&self) -> Users {
        let a = self.clone();
        Users {
            uid: a.updated_by,
            username: a.updated_by_username,
            nickname: a.updated_by_nickname,
            password: "".to_string(),
            status: 0,
            email: "".to_string(),
            admin: false,
            intro: "".to_string(),
            last_login_on: DateTimeNative::now(),
            salt: "".to_string(),
            must_change_password: false,
            password_changed_on: 0,
            created_at: DateTimeNative::now(),
            updated_at: DateTimeNative::now()
        }
    }

    async fn created_at(&self) -> String {
        self.created_at.format(&DateFormat::Normal.to_string()).to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.format(&DateFormat::Normal.to_string()).to_string()
    }

    async fn author(&self) -> Users {
        let n = self.clone();
        Users {
            uid: n.uid,
            username: n.author_username,
            nickname: n.author_nickname,
            password: "".to_string(),
            status: 0,
            email: "".to_string(),
            admin: false,
            intro: "".to_string(),
            last_login_on: DateTimeNative::now(),
            salt: "".to_string(),
            must_change_password: false,
            password_changed_on: 0,
            created_at: DateTimeNative::now(),
            updated_at: DateTimeNative::now()
        }
    }

    async fn category(&self) -> Taxonomies {
        let n = self.clone();
        Taxonomies {
            tid: n.tid,
            vid: n.category_vid,
            pid: 0,
            bundle: n.category_bundle,
            name: n.category_name,
            description: "".to_string(),
            description_format: "".to_string(),
            weight: 0,
            count: 0
        }
    }

    async fn node_body(&self) -> NodeBody {
        let n = self.clone();
        NodeBody {
            nid: n.nid,
            summary: n.summary,
            body: n.body,
            body_format: n.body_format
        }
    }

    async fn tags(
        &self,
        ctx: &Context<'_>
    ) -> Result<Vec<Taxonomies>, String> {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        let bundle = TaxonomyBundle::Tag.to_string();
        let res = find_node_taxonomies(rb.clone(), &bundle, &self.nid)
            .await;

        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(format!("No tags, {}", err.to_string()))
        }
    }
}