use async_graphql::{Object, MergedObject};
use serde::{Deserialize, Serialize};
use crate::typings::{
    DetailNode,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageInfo {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i32,
}

#[Object]
impl PageInfo {
    async fn page(&self) -> i32 {
        self.page
    }
    async fn page_size(&self) -> i32 {
        self.page_size
    }
    async fn total_count(&self) -> i32 {
        self.total_count
    }
}