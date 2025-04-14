use crate::entities::prelude::*;
use serde::Serialize;

/// 获取用户详细信息返回
#[derive(Debug, Serialize)]
pub struct DescribeUserDetailResponse {
    pub user: UserModel,
}