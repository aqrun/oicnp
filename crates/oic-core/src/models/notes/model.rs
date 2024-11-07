use loco_rs::prelude::*;
use crate::entities::prelude::*;
use serde_json::json;
use super::CreateNoteReqParams;

#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}

impl NoteModel {
    /// 创建 note
    pub async fn create(db: &DatabaseConnection, params: &CreateNoteReqParams) -> ModelResult<Self> {
        let mut item = NoteActiveModel {
            ..Default::default()
        };

        item.set_from_json(json!(params))?;
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 批量创建 note
    pub async fn create_multi(db: &DatabaseConnection, params: &[CreateNoteReqParams]) -> ModelResult<i64> {
        let txn = db.begin().await?;
        let mut notes: Vec<NoteActiveModel> = Vec::new();

        for item in params.iter() {
            match NoteActiveModel::from_json(json!(item)) {
                Ok(note) => {
                    notes.push(note);
                },
                Err(err) => {
                    txn.rollback().await?;
                    return Err(ModelError::DbErr(err));
                }
            };
        }
        
        let res = NoteEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(res.last_insert_id)
    }
}