use crate::{
    entities::prelude::*,
    typings::ListData,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait};
use validator::Validate;
use super::{CreateNoteReqParams, NoteFilters, UpdateNoteReqParams, DeleteNoteReqParams};


#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for NoteModel {
    type CreateReqParams = CreateNoteReqParams;

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }
        
        let txn = db.begin().await?;
        let mut notes: Vec<NoteActiveModel> = Vec::new();

        for item in params.iter() {
            let mut note = NoteActiveModel {
                ..Default::default()
            };
    
            item.update(&mut note);
            item.update_by_create(&mut note);
            notes.push(note);
        }
        
        let _ = NoteEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量note添加完成"))
    }
}

impl NoteModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = NoteEntity::find()
            .filter(NoteColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ////
    /// 获取note列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: NoteFilters) -> ModelResult<ListData<NoteModel>> {
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
    pub async fn create(db: &DatabaseConnection, params: &CreateNoteReqParams) -> ModelResult<Self> {
        catch_err(params.validate())?;

        let mut item = NoteActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateNoteReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.id)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteNoteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = NoteEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}