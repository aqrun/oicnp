use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListData<T> {
    pub data: Vec<T>,
    /// 第几页
    pub page: u64,
    /// 每页数据量
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
    /// 数据总条数
    pub total_count: u64,
}
