use async_trait::async_trait;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use crate::utils::catch_err;
use crate::entities::poetry::*;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder};

use super::{
    PoetryFilters,
    CreatePoetryReqParams,
    UpdatePoetryReqParams,
    DeletePoetryReqParams,
};

#[async_trait]
impl ModelCrudHandler for PoetryModel {
    type DataModel = Self;
    type FilterParams = PoetryFilters;
    type CreateReqParams = CreatePoetryReqParams;
    type UpdateReqParams = UpdatePoetryReqParams;
    type DeleteReqParams = DeletePoetryReqParams;

    /// 根据ID查找一个
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self::DataModel> {
        let poetry = PoetryEntity::find()
            .filter(
                model::query::condition()
                    .eq(PoetryColumn::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        poetry.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 根据vid查找一个
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self::DataModel> {
        let poetry = PoetryEntity::find()
            .filter(
                model::query::condition()
                    .eq(PoetryColumn::Uuid, vid)
                    .build(),
            )
            .one(db)
            .await?;
        poetry.ok_or_else(|| ModelError::EntityNotFound)
    }

    ////
    /// 获取poetry列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = PoetryEntity::find();

        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(PoetryColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Uuid.eq(x));
            }
        }

        if let Some(x) = &params.title {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Title.contains(x));
            }
        }

        if let Some(x) = &params.author_id {
            if *x > 0 {
                q = q.filter(PoetryColumn::AuthorId.eq(*x));
            }
        }

        if let Some(x) = &params.dynasty {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Dynasty.eq(x));
            }
        }

        if let Some(x) = &params.weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::Weight.eq(*x));
            }
        }

        if let Some(x) = &params.hot_weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::HotWeight.eq(*x));
            }
        }

        if let Some(x) = &params.content {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Content.contains(x));
            }
        }

        if let Some(x) = &params.word_count {
            if *x > 0 {
                q = q.filter(PoetryColumn::WordCount.eq(*x));
            }
        }

        if let Some(x) = &params.tags {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Tags.contains(x));
            }
        }

        let mut order_by = PoetryColumn::Id;

        if order_by_str.eq("created_at") {
            order_by = PoetryColumn::CreatedAt;
        } else if order_by_str.eq("weight") {
            order_by = PoetryColumn::Weight;
        } else if order_by_str.eq("hot_weight") {
            order_by = PoetryColumn::HotWeight;
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
        let mut poetries: Vec<PoetryActiveModel> = Vec::new();

        for item in params.iter() {
            let mut poetry = PoetryActiveModel::new();
            item.update(&mut poetry);
            item.update_by_create(&mut poetry);

            poetries.push(poetry);
        }

        let _ = PoetryEntity::insert_many(poetries).exec(&txn).await?;

        txn.commit().await?;
        
        Ok(String::from("批量poetry添加完成"))
    }

    /// 创建 poetry
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut poetry = PoetryActiveModel::new();
        params.update(&mut poetry);
        params.update_by_create(&mut poetry);

        let poetry = poetry.insert(db).await?;

        Ok(poetry.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", id)));
        }

        let mut poetry = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();    
        params.update(&mut poetry);
    
        let item = poetry.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Message(format!("数据不存在, id: {}", id)));
        }

        let _res = PoetryEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl PoetryModel {

}

impl PoetryActiveModel {
    
}
