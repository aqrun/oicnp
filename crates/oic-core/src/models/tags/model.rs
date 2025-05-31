use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait};
use validator::Validate;
use super::{CreateTagReqParams, TagFilters, UpdateTagReqParams, DeleteTagReqParams};


#[async_trait::async_trait]
impl ActiveModelBehavior for TagActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for TagModel {
    type DataModel = Self;
    type FilterParams = TagFilters;
    type CreateReqParams = CreateTagReqParams;
    type UpdateReqParams = UpdateTagReqParams;
    type DeleteReqParams = DeleteTagReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = TagEntity::find()
            .filter(TagColumn::TagId.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据VID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
        if vid.is_empty() {
            return Err(ModelError::Any(format!("数据不存在,vid: {}", vid).into()));
        }

        let item = TagEntity::find()
            .filter(TagColumn::TagVid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,vid: {}", vid).into())
        })
    }

    ////
    /// 获取note列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = TagEntity::find();

        if let Some(x) = params.tag_id {
            if x > 0 {
                q = q.filter(TagColumn::TagId.eq(x));
            }
        }

        if let Some(x) = &params.tag_name {
            if !x.is_empty() {
                q = q.filter(TagColumn::TagName.contains(x));
            }
        }

        let mut order_by = TagColumn::TagId;

        if order_by_str.eq("tag_name") {
            order_by = TagColumn::TagName;  
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        Ok((list, total))
    }

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }

        for item in params.iter() {
            Self::upsert(db, item).await?;
        }

        Ok(String::from("批量tag添加完成"))
    }

    /// 创建 tag
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = TagActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.tag_id)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.tag_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.tag_id)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.tag_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = TagEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}

impl TagModel {
    /// 更新标签计数
    pub async fn update_count_by_id(db: &DatabaseConnection, tag_id: i64) -> ModelResult<()> {
        let tag_model = Self::find_by_id(db, tag_id)
            .await?;

        let count = tag_model.tag_count;
        let mut tag = tag_model.into_active_model();
        tag.tag_count = Set(count + 1);

        tag.update(db).await?;

        Ok(())
    }

    /// 创建或更新标签
    pub async fn upsert(db: &DatabaseConnection, params: &CreateTagReqParams) -> ModelResult<i64> {
        if let Some(tag_vid) = &params.tag_vid {
            match Self::find_by_vid(db, tag_vid.as_str()).await {
                Ok(existing_tag) => {
                    let count = existing_tag.tag_count;
                    // Tag exists, update count
                    let mut tag = existing_tag.into_active_model();
                    tag.tag_count = Set(count + 1);
                    let updated_tag = tag.update(db).await?;
                    Ok(updated_tag.tag_id)
                }
                Err(_) => {
                    // Tag doesn't exist, create new one
                    Self::create(db, params).await
                }
            }
        } else {
            Err(ModelError::Any("tag_vid is required".into()))
        }
    }
}