use loco_rs::prelude::*;
use crate::{entities::prelude::*, utils::model_err};
use serde_json::json;
use super::{CreateNoteReqParams, QueryNoteListReqParams};
use anyhow::anyhow;

#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}

impl NoteModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        let item = NoteEntity::find()
            .filter(NoteColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            model_err(anyhow!("没找到数据,id: {}", id))
        })
    }

    pub async fn find_list(db: &DatabaseConnection, params: QueryNoteListReqParams) -> ModelResult<Vec<Self>> {
        let mut q = NoteEntity::find();

        if let Some(x) = params.id {
            if x > 0 {
                q = q.filter(NoteColumn::Id.eq(x));
            }
        }

        if let Some(x) = params.title {
            if !x.is_empty() {
                q = q.filter(NoteColumn::Title.contains(&x));
            }
        }

        let res = q.all(db)
            .await?;

        Ok(res)
    }

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