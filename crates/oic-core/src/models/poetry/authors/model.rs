use async_trait::async_trait;
use crate::utils::catch_err;
use crate::entities::poetry::*;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder};
use validator::Validate;

use super::{
    AuthorFilters,
    CreateAuthorReqParams,
    UpdateAuthorReqParams,
    DeleteAuthorReqParams,
};

#[async_trait]
impl ModelCrudHandler for AuthorModel {
    type DataModel = Self;
    type FilterParams = AuthorFilters;
    type CreateReqParams = CreateAuthorReqParams;
    type UpdateReqParams = UpdateAuthorReqParams;
    type DeleteReqParams = DeleteAuthorReqParams;

    /// 根据ID查找一个
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self::DataModel> {
        let author = AuthorEntity::find()
            .filter(AuthorColumn::Id.eq(id))
            .one(db)
            .await?;
        author.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 根据vid查找一个
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self::DataModel> {
        let author = AuthorEntity::find()
            .filter(AuthorColumn::Uuid.eq(vid))
            .one(db)
            .await?;
        author.ok_or_else(|| ModelError::EntityNotFound)
    }

    ////
    /// 获取author列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = AuthorEntity::find();

        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(AuthorColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                q = q.filter(AuthorColumn::Uuid.eq(x));
            }
        }

        let mut order_by = AuthorColumn::Id;

        if order_by_str.eq("created_at") {
            order_by = AuthorColumn::CreatedAt;
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
        let mut authors: Vec<AuthorActiveModel> = Vec::new();

        for item in params.iter() {
            let mut author = AuthorActiveModel::new();
            item.update(&mut author);
            item.update_by_create(&mut author);

            authors.push(author);
        }

        let _ = AuthorEntity::insert_many(authors).exec(&txn).await?;

        txn.commit().await?;
        
        Ok(String::from("批量author添加完成"))
    }

    /// 创建 author
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut author = AuthorActiveModel::new();
        params.update(&mut author);
        params.update_by_create(&mut author);

        let author = author.insert(db).await?;

        Ok(author.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", id)));
        }

        let mut author = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();    
        params.update(&mut author);
    
        let item = author.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Message(format!("数据不存在, id: {}", id)));
        }

        let _res = AuthorEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl AuthorModel {
    pub async fn upsert(db: &DatabaseConnection, params: &CreateAuthorReqParams) -> ModelResult<i32> {
        let author = AuthorEntity::find()
            .filter(AuthorColumn::Name.eq(params.name.as_ref().unwrap()))
            .one(db)
            .await?;

        // 如果存在，则更新
        if let Some(author) = author {
            let mut author = author.into_active_model();
            params.update(&mut author);
            let a = author.update(db).await?;
            return Ok(a.id);
        }

        // 如果不存在，则创建
        match Self::create(db, params).await {
            Ok(id) => {
                Ok(id as i32)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl AuthorActiveModel {
    
}
