use async_trait::async_trait;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use crate::utils::catch_err;
use crate::entities::poetry::*;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder};

use super::{
    ChapterFilters,
    CreateChapterReqParams,
    UpdateChapterReqParams,
    DeleteChapterReqParams,
};

#[async_trait]
impl ModelCrudHandler for ChapterModel {
    type DataModel = Self;
    type FilterParams = ChapterFilters;
    type CreateReqParams = CreateChapterReqParams;
    type UpdateReqParams = UpdateChapterReqParams;
    type DeleteReqParams = DeleteChapterReqParams;

    /// 根据ID查找一个
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self::DataModel> {
        let chapter = ChapterEntity::find()
            .filter(
                model::query::condition()
                    .eq(ChapterColumn::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        chapter.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 根据vid查找一个
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self::DataModel> {
        let chapter = ChapterEntity::find()
            .filter(
                model::query::condition()
                    .eq(ChapterColumn::Uuid, vid)
                    .build(),
            )
            .one(db)
            .await?;
        chapter.ok_or_else(|| ModelError::EntityNotFound)
    }

    ////
    /// 获取chapter列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = ChapterEntity::find();

        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(ChapterColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                q = q.filter(ChapterColumn::Uuid.eq(x));
            }
        }

        if let Some(x) = &params.pid {
            if *x > 0 {
                q = q.filter(ChapterColumn::Pid.eq(*x));
            }
        }

        if let Some(x) = &params.poetry_id {
            if *x > 0 {
                q = q.filter(ChapterColumn::PoetryId.eq(*x));
            }
        }

        if let Some(x) = &params.title {
            if !x.is_empty() {
                q = q.filter(ChapterColumn::Title.contains(x));
            }
        }

        if let Some(x) = &params.content {
            if !x.is_empty() {
                q = q.filter(ChapterColumn::Content.contains(x));
            }
        }

        if let Some(x) = &params.word_count {
            if *x > 0 {
                q = q.filter(ChapterColumn::WordCount.eq(*x));
            }
        }

        if let Some(x) = &params.weight {
            if *x > 0 {
                q = q.filter(ChapterColumn::Weight.eq(*x));
            }
        }

        let mut order_by = ChapterColumn::Id;

        if order_by_str.eq("created_at") {
            order_by = ChapterColumn::CreatedAt;
        } else if order_by_str.eq("weight") {
            order_by = ChapterColumn::Weight;
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        Ok((list, total))
    }

    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }

        let txn = db.begin().await?;
        let mut chapters: Vec<ChapterActiveModel> = Vec::new();

        for item in params.iter() {
            let mut chapter = ChapterActiveModel::new();
            item.update(&mut chapter);
            item.update_by_create(&mut chapter);

            chapters.push(chapter);
        }

        let _ = ChapterEntity::insert_many(chapters).exec(&txn).await?;

        txn.commit().await?;
        
        Ok(String::from("批量chapter添加完成"))
    }

    /// 创建 chapter
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut chapter = ChapterActiveModel::new();
        params.update(&mut chapter);
        params.update_by_create(&mut chapter);

        let chapter = chapter.insert(db).await?;

        Ok(chapter.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", id)));
        }

        let mut chapter = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();    
        params.update(&mut chapter);
    
        let item = chapter.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Message(format!("数据不存在, id: {}", id)));
        }

        let _res = ChapterEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl ChapterModel {
    
}

impl ChapterActiveModel {
    
}

