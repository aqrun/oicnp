use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Condition, QuerySelect};
use validator::Validate;
use super::{
    CreateCacheReqParams, CacheFilters, UpdateCacheReqParams, DeleteCacheReqParams,
    CacheScopeModel,
    PartialCacheModel,
};
use crate::{
    utils::utc_now, constants::DATE_TIME_FORMAT,
    models::attribute_values::AttributeValueFilters,
};

#[async_trait::async_trait]
impl ActiveModelBehavior for CacheActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for CacheModel {
    type DataModel = Self;
    type FilterParams = CacheFilters;
    type CreateReqParams = CreateCacheReqParams;
    type UpdateReqParams = UpdateCacheReqParams;
    type DeleteReqParams = DeleteCacheReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = CacheEntity::find()
            .filter(CacheColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, key: &str) -> ModelResult<Self> {
        let item = CacheEntity::find()
            .filter(CacheColumn::CacheKey.eq(key))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,cache_key: {}", key).into())
        })
    }

    ////
    /// 获取列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = CacheEntity::find();

        if let Some(x) = params.id {
            if x > 0 {
                q = q.filter(CacheColumn::Id.eq(x));
            }
        }

        if let Some(x) = &params.cache_key {
            if !x.is_empty() {
                q = q.filter(CacheColumn::CacheKey.eq(x));
            }
        }

        if let Some(x) = &params.scope {
            if !x.is_empty() {
                q = q.filter(CacheColumn::Scope.eq(x));
            }
        }

        let mut order_by = CacheColumn::Id;

        if order_by_str.eq("cache_key") {
            order_by = CacheColumn::CacheKey;
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
        catch_err(params.validate())?;
        
        for item in params.iter() {
            let mut cache = CacheActiveModel {
                ..Default::default()
            };
    
            item.update(&mut cache);
            item.update_by_create(&mut cache);

            let _ = cache.insert(db).await?;
        }

        Ok(String::from("批量cache添加完成"))
    }

    /// 创建
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = CacheActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.id)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
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
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let mut q = CacheEntity::delete_many();

        if let Some(x) = &params.id {
            q = q.filter(CacheColumn::Id.eq(*x));
        }

        if let Some(x) = &params.cache_key {
            q = q.filter(CacheColumn::CacheKey.eq(x));
        }

        let _res = q.exec(db).await?;

        Ok(0)
    }
}

impl CacheModel {
    pub async fn find_scope_list(db: &DatabaseConnection) -> ModelResult<Vec<CacheScopeModel>> {
        let value_filters = AttributeValueFilters {
            vid: Some(String::from("cache_scope")),
            ..Default::default()
        };
        let (attr_values, _) = AttributeValueModel::find_list(db, &value_filters).await?;

        let q = CacheEntity::find()
            .select_only()
            .columns([CacheColumn::Scope])
            .group_by(CacheColumn::Scope);
            
        let list = q.into_model::<PartialCacheModel>()
            .all(db)
            .await?;

        let scopes = list.iter().map(|scope_only| {
            let mut scope = String::from("");
            let mut label = String::from("其它");

            if let Some(x) = &scope_only.scope {
                scope = String::from(x);
            }

            let attr_value = attr_values.iter().find(|attr_value| attr_value.value == scope);
            
            if let Some(attr_value) = attr_value {
                label = String::from(attr_value.label.as_str());
            }

            CacheScopeModel {
                scope,
                label,
            }
        }).collect::<Vec<CacheScopeModel>>();

        Ok(scopes)
    }

    /// 删除数据
    pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<i64> {
        let q = CacheEntity::delete_many()
            .filter(CacheColumn::Id.eq(id));

        let _res = q.exec(db).await?;

        Ok(0)
    }

    /// 删除数据
    pub async fn delete_by_key(db: &DatabaseConnection, key: &str) -> ModelResult<i64> {
        let q = CacheEntity::delete_many()
            .filter(CacheColumn::CacheKey.eq(key));

        let _res = q.exec(db).await?;

        Ok(0)
    }

    pub async fn delete_all(db: &DatabaseConnection) -> ModelResult<i64> {
        let q = CacheEntity::delete_many()
            .filter(CacheColumn::Id.gt(0));

        let _res = q.exec(db).await?;

        Ok(0)
    }

    ///
    /// 数据刷新
    /// 清空全部过期数据
    /// 
    pub async fn refresh(db: &DatabaseConnection) -> ModelResult<()> {
        let q = CacheEntity::delete_many()
            .filter(
                Condition::all()
                    .add(CacheColumn::ExpiredAt.lt(utc_now()))
                    .add(CacheColumn::ExpiredAt.is_not_null())
            );

        let _res = q.exec(db).await?;

        Ok(())
    }
}