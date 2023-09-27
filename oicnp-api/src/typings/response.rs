use async_graphql::{Object, OutputType};
use serde::{Deserialize, Serialize};
use std::marker::{Send, Sync};

///
/// 分页数据信息
///
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

///
/// 返回分页列表数据
///
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

///
/// 返回单个数据信息
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OicRes<T> {
    pub code: String,
    pub message: Option<String>,
    pub is_success: bool,
    pub data: Option<T>,
}

#[Object]
impl<T> OicRes<T> where T: Send + Sync + OutputType {
    async fn code(&self) -> &str {
        self.code.as_str()
    }
    async fn message(&self) -> &Option<String> {
        &self.message
    }
    async fn is_success(&self) -> bool {
        self.is_success
    }
    async fn data(&self) -> &Option<T> {
        &self.data
    }
}

