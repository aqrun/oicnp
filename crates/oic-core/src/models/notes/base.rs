use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateNoteReqParams {
    #[validate(required, length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "content 最少2个字符"))]
    pub content: Option<String>,
}

///
/// 更新 note 参数
/// 
#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct UpdateNoteReqParams {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>,
}