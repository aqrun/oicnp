use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder};
use validator::Validate;
use super::{CreateIpReqParams, IpFilters, UpdateIpReqParams, DeleteIpReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for IpActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for IpModel {
    type DataModel = Self;
    type FilterParams = IpFilters;
    type CreateReqParams = CreateIpReqParams;
    type UpdateReqParams = UpdateIpReqParams;
    type DeleteReqParams = DeleteIpReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = IpEntity::find()
            .filter(IpColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, ip: &str) -> ModelResult<Self> {
        let item = IpEntity::find()
            .filter(IpColumn::Ip.eq(ip))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,ip: {}", ip).into())
        })
    }

    ////
    /// 获取node列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = IpEntity::find();

        if let Some(x) = params.id {
            if x > 0 {
                q = q.filter(IpColumn::Id.eq(x));
            }
        }

        if let Some(x) = &params.ip {
            if !x.is_empty() {
                q = q.filter(IpColumn::Ip.contains(x));
            }
        }

        let mut order_by = IpColumn::Id;

        if order_by_str.eq("ip") {
            order_by = IpColumn::Ip;
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
            let mut target = IpActiveModel {
                ..Default::default()
            };

            item.update(&mut target);
            item.update_by_create(&mut target);

            let _ = target.insert(db).await?;
        }

        Ok(String::from("批量ip添加完成"))
    }

    /// 创建 node
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = IpActiveModel {
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
        let nid = params.id.unwrap_or(0);

        if nid <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", nid).into()));
        }

        let _res = IpEntity::delete_by_id(nid)
            .exec(db)
            .await?;

        Ok(nid)
    }
}

impl IpModel {
    pub async fn upsert(db: &DatabaseConnection, params: &CreateIpReqParams) -> ModelResult<i64> {
        let res = match Self::update(db, params).await {
            Ok(id) => {
                Ok(id)
            },
            _ => {
                Self::create(db, params).await
            }
        };

        res
    }
}