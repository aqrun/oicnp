use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::entities::prelude::*;
use validator::Validate;

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CreateNoteReqParams {
    #[validate(required, length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "content 最少2个字符"))]
    pub content: Option<String>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}

impl NoteModel {
    /// 创建 note
    pub async fn insert(db: &DatabaseConnection, params: &CreateNoteReqParams) -> ModelResult<Self> {
        let mut item = NoteActiveModel {
            ..Default::default()
        };

        if params.title.is_some() {
            item.title = Set(params.title.clone());
        }

        if params.content.is_some() {
            item.title = Set(params.content.clone());
        }
    
        let item = item.insert(db).await?;

        Ok(item)
    }
}