use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder};
use validator::Validate;
use super::{CreateCategoryReqParams, CategoryFilters, UpdateCategoryReqParams, DeleteCategoryReqParams};


#[async_trait::async_trait]
impl ActiveModelBehavior for CategoryActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for CategoryModel {
    type DataModel = Self;
    type FilterParams = CategoryFilters;
    type CreateReqParams = CreateCategoryReqParams;
    type UpdateReqParams = UpdateCategoryReqParams;
    type DeleteReqParams = DeleteCategoryReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = CategoryEntity::find()
            .filter(CategoryColumn::CatId.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
        if vid.is_empty() {
            return Err(ModelError::Any(format!("vid为空: {}", vid).into()));
        }

        let item = CategoryEntity::find()
            .filter(CategoryColumn::CatVid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在, vid: {}", vid).into())
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

        let mut q = CategoryEntity::find();

        if let Some(x) = params.cat_id {
            if x > 0 {
                q = q.filter(CategoryColumn::CatId.eq(x));
            }
        }

        if let Some(x) = &params.cat_name {
            if !x.is_empty() {
                q = q.filter(CategoryColumn::CatName.contains(x));
            }
        }

        let mut order_by = CategoryColumn::CatId;

        if order_by_str.eq("cat_name") {
            order_by = CategoryColumn::CatName;  
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

        // 缓存已存在的权限数据
        let mut exist_categories: HashMap<String, Self> = HashMap::new();

        for item in params.iter() {
            // 先使用缓存父菜单数据
            let mut parent_category: Option<Self> = None;
            let mut parent_vid = String::from("");

            if let Some(x) = &item.parent_vid {
                parent_vid = String::from(x);
            }
            
            if !parent_vid.is_empty() {
                let res = exist_categories.get(parent_vid.as_str());

                if let Some(res) = res {
                    parent_category = Some(res.clone());
                } else {
                    // 不存在从数据库读取
                    if let Ok(res) = Self::find_by_vid(db, parent_vid.as_str()).await {
                        exist_categories.insert(String::from(res.cat_vid.as_str()), res.clone());
                        parent_category = Some(res);
                    }
                }
            }

            let mut category = CategoryActiveModel {
                ..Default::default()
            };

            item.update(&mut category);
            item.update_by_create(&mut category);

            if let Some(parent_category) = parent_category {
                category.cat_pid = Set(parent_category.cat_id);
            }

            let category_model = category.insert(db).await?;
 
            // 添加缓存数据
            exist_categories.insert(String::from(category_model.cat_vid.as_str()), category_model);
        }

        Ok(String::from("批量category添加完成"))
    }

    /// 创建 category
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = CategoryActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.cat_id)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.cat_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.cat_id)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.cat_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = CategoryEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}

impl NoteModel {  
}