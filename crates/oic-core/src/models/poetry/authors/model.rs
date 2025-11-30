use async_trait::async_trait;
use crate::utils::catch_err;
use crate::entities::poetry::*;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder, QueryFilter};
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

        if let Some(x) = &params.name {
            if !x.is_empty() {
                q = q.filter(AuthorColumn::Name.contains(x));
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

        let fields = match &params.fields {
            Some(x) => String::from(x),
            None => String::from(""),
        };

        let list = list.into_iter().map(|x| {
            let mut author = AuthorModel {
                id: x.id,
                name: String::from(x.name.as_str()),
                count: x.count,
                dynasty: String::from(x.dynasty.as_str()),
                ..Default::default()
            };

            if fields.contains("description") {
                author.description = String::from(x.description.as_str());
            }
            if fields.contains("short_description") {
                author.short_description = String::from(x.short_description.as_str());
            }
            if fields.contains("birth_at") {
                author.birth_at = x.birth_at;
            }
            if fields.contains("death_at") {
                author.death_at = x.death_at;
            }
            if fields.contains("weight") {
                author.weight = x.weight;
            }
            if fields.contains("created_at") {
                author.created_at = x.created_at;
            }
            if fields.contains("updated_at") {
                author.updated_at = x.updated_at;
            }

            author
        }).collect();

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
            // 同时使用 name 和 dynasty 查询
            .filter(AuthorColumn::Name.eq(params.name.as_ref().unwrap()))
            .filter(AuthorColumn::Dynasty.eq(params.dynasty.as_ref().unwrap()))
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

    /// 更新作者作品计数 + 1
    pub async fn update_count_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<()> {
        if id <= 0 {
            return Ok(());
        }

        let author_model = match Self::find_by_id(db, id as i64)
            .await {
                Ok(author) => author,
                Err(_e) => {
                    return Ok(());
                }
            };
        let count = author_model.count;
        let mut author = author_model.into_active_model();
        author.count = Set(count + 1);
        let _ = match author.update(db).await {
            Ok(data) => data,
            Err(_e) => {
                return Ok(());
            }
        };

        Ok(())
    }
}

impl AuthorActiveModel {
    
}
