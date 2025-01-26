use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    typings::ListData,
};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Set, TransactionTrait};
use serde_json::json;
use validator::Validate;
use super::{CreateNoteReqParams, NoteFilters, UpdateNoteReqParams, DeleteNoteReqParams};
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}

impl NoteModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Self> {
        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let item = NoteEntity::find()
            .filter(NoteColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在,id: {}", id)
        })
    }

    ////
    /// 获取note列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: NoteFilters) -> Result<ListData<NoteModel>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

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

        let mut order_by = NoteColumn::Id;

        if order_by_str.eq("title") {
            order_by = NoteColumn::Title;
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        let res = ListData {
            data: list,
            page,
            page_size,
            total,
        };

        Ok(res)
    }

    /// 创建 note
    pub async fn create(db: &DatabaseConnection, params: &CreateNoteReqParams) -> Result<Self> {
        let _ = catch_err(params.validate())?;

        let mut item = NoteActiveModel {
            ..Default::default()
        };

        item.set_from_json(json!(params))?;
        item.created_at = Set(Some(utc_now()));
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 批量创建 note
    pub async fn create_multi(db: &DatabaseConnection, params: &[CreateNoteReqParams]) -> Result<String> {
        for item in params {
            let _ = catch_err(item.validate())?;
        }
        
        let txn = db.begin().await?;
        let mut notes: Vec<NoteActiveModel> = Vec::new();

        for item in params.iter() {
            match NoteActiveModel::from_json(json!(item)) {
                Ok(mut note) => {
                    if note.created_at.is_not_set() {
                        note.created_at = Set(Some(utc_now()));
                    }
                    
                    notes.push(note);
                },
                Err(err) => {
                    txn.rollback().await?;
                    return Err(anyhow!("批量数据有误, {}", err));
                }
            };
        }
        
        let _ = NoteEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量note添加完成"))
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateNoteReqParams) -> Result<i64> {
        let _ = catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let mut item = Self::find_by_id(&db, id)
            .await?
            .into_active_model();

        item.set_from_json(json!(params))?;
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.update(db).await?;

        Ok(item.id)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteNoteReqParams) -> Result<i64> {
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let _res = NoteEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}