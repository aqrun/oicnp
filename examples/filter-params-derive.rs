//! 筛选参数相关宏功能测试
//! 
//! cargo expand --example filter-params-derive
//! 

use oic_derives::{FilterParams, add_filter_fields};
use serde::Deserialize;

#[add_filter_fields]
#[derive(FilterParams, Deserialize)]
pub struct ExampleNoteFilters {
    pub id: Option<i64>,
    pub title: Option<String>,
}

fn main() {

}
