use async_graphql::{Object, MergedObject, OutputType};
use crate::typings::{
    DetailNode,
};
use serde::{Deserialize, Serialize};
use std::marker::{Send, Sync};
use oicnp_core::{
    typings::{ListData},
    models::{DetailNode as CoreDetailNode},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagerInfo {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i32,
}

#[Object]
impl PagerInfo {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResListData<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub total_count: u64,
}

#[Object]
impl<T> ResListData<T>
    where T: Send + Sync + OutputType
{
    async fn data(&self) -> &Vec<T> {
        &self.data
    }
    async fn page(&self) -> u64 {
        self.page
    }
    async fn page_size(&self) -> u64 {
        self.page_size
    }
    async fn total_count(&self) -> u64 {
        self.total_count
    }
}
