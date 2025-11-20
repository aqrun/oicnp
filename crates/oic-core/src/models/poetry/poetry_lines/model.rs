use async_trait::async_trait;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use crate::utils::catch_err;
use crate::entities::poetry::*;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder};

use super::{
    PoetryLineFilters,
    CreatePoetryLineReqParams,
    UpdatePoetryLineReqParams,
    DeletePoetryLineReqParams,
};

#[async_trait]
impl ModelCrudHandler for PoetryLineModel {
    type DataModel = Self;
    type FilterParams = PoetryLineFilters;
    type CreateReqParams = CreatePoetryLineReqParams;
    type UpdateReqParams = UpdatePoetryLineReqParams;
    type DeleteReqParams = DeletePoetryLineReqParams;

    /// 根据ID查找一个
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self::DataModel> {
        let poetry_line = PoetryLineEntity::find()
            .filter(
                model::query::condition()
                    .eq(PoetryLineColumn::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        poetry_line.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 根据vid查找一个
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self::DataModel> {
        // PoetryLine 没有 uuid 字段，使用 id 作为 vid
        if let Ok(id) = vid.parse::<i64>() {
            Self::find_by_id(db, id).await
        } else {
            Err(ModelError::EntityNotFound)
        }
    }

    ////
    /// 获取poetry_line列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = PoetryLineEntity::find();

        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(PoetryLineColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.poetry_id {
            if *x > 0 {
                q = q.filter(PoetryLineColumn::PoetryId.eq(*x));
            }
        }

        if let Some(x) = &params.line_number {
            if *x > 0 {
                q = q.filter(PoetryLineColumn::LineNumber.eq(*x));
            }
        }

        if let Some(x) = &params.content {
            if !x.is_empty() {
                q = q.filter(PoetryLineColumn::Content.contains(x));
            }
        }

        if let Some(x) = &params.pinyin {
            if !x.is_empty() {
                q = q.filter(PoetryLineColumn::Pinyin.contains(x));
            }
        }

        if let Some(x) = &params.description {
            if !x.is_empty() {
                q = q.filter(PoetryLineColumn::Description.contains(x));
            }
        }

        if let Some(x) = &params.notes {
            if !x.is_empty() {
                q = q.filter(PoetryLineColumn::Notes.contains(x));
            }
        }

        let mut order_by = PoetryLineColumn::Id;

        if order_by_str.eq("created_at") {
            order_by = PoetryLineColumn::CreatedAt;
        } else if order_by_str.eq("line_number") {
            order_by = PoetryLineColumn::LineNumber;
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
        let mut poetry_lines: Vec<PoetryLineActiveModel> = Vec::new();

        for item in params.iter() {
            let mut poetry_line = PoetryLineActiveModel::new();
            item.update(&mut poetry_line);
            item.update_by_create(&mut poetry_line);

            poetry_lines.push(poetry_line);
        }

        let _ = PoetryLineEntity::insert_many(poetry_lines).exec(&txn).await?;

        txn.commit().await?;
        
        Ok(String::from("批量poetry_line添加完成"))
    }

    /// 创建 poetry_line
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut poetry_line = PoetryLineActiveModel::new();
        params.update(&mut poetry_line);
        params.update_by_create(&mut poetry_line);

        let poetry_line = poetry_line.insert(db).await?;

        Ok(poetry_line.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", id)));
        }

        let mut poetry_line = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();    
        params.update(&mut poetry_line);
    
        let item = poetry_line.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Message(format!("数据不存在, id: {}", id)));
        }

        let _res = PoetryLineEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl PoetryLineModel {

}

impl PoetryLineActiveModel {
    
}

