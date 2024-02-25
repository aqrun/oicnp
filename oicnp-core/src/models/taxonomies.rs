use crate::entities::prelude::*;
use async_graphql::Object;

#[Object]
impl TagModel {
    async fn id(&self) -> i64 {
        self.tag_id
    }
    async fn vid(&self) -> &str {
        self.tag_vid.as_str()
    }
    async fn name(&self) -> &str {
        self.tag_name.as_str()
    }
    async fn weight(&self) -> i32 {
        self.weight
    }
    async fn count(&self) -> i64 {
        self.tag_count
    }
}

#[Object]
impl CategoryModel {
    async fn cat_tid(&self) -> i64 {
        self.cat_id
    }
    async fn cat_vid(&self) -> &str {
        self.cat_vid.as_str()
    }
    async fn cat_pid(&self) -> i64 {
        self.cat_pid
    }
    async fn cat_name(&self) -> &str {
        self.cat_name.as_str()
    }
    async fn cat_desc(&self) -> &str {
        self.cat_desc.as_str()
    }
    async fn cat_desc_format(&self) -> &str {
        self.cat_desc_format.as_str()
    }
    async fn weight(&self) -> i32 {
        self.weight
    }
}

#[derive(Clone, Debug)]
pub struct NewCategory {
    pub vid: String,
    pub pid: i64,
    pub name: String,
    pub desc: String,
    pub desc_format: String,
    pub weight: i32,
}

#[derive(Clone, Debug)]
pub struct NewTag {
    pub vid: String,
    pub name: String,
    pub weight: i32,
    pub count: i32,
}