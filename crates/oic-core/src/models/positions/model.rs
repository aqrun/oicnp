use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait};
use validator::Validate;
use super::{CreatePositionReqParams, PositionFilters, UpdatePositionReqParams, DeletePositionReqParams};


#[async_trait::async_trait]
impl ActiveModelBehavior for PositionActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for PositionModel {
    type DataModel = Self;
    type FilterParams = PositionFilters;
    type CreateReqParams = CreatePositionReqParams;
    type UpdateReqParams = UpdatePositionReqParams;
    type DeleteReqParams = DeletePositionReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = PositionEntity::find()
            .filter(PositionColumn::PositionId.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(_db: &DatabaseConnection, _vid: &str) -> ModelResult<Self> {
        Ok(Self::default())
    }

    ////
    /// 获取部门列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = PositionEntity::find();

        if let Some(x) = params.position_id {
            if x > 0 {
                q = q.filter(PositionColumn::PositionId.eq(x));
            }
        }

        if let Some(x) = &params.name {
            if !x.is_empty() {
                q = q.filter(PositionColumn::Name.contains(x));
            }
        }

        let mut order_by = PositionColumn::PositionId;

        if order_by_str.eq("name") {
            order_by = PositionColumn::Name;
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
        
        let txn = db.begin().await?;
        let mut positions: Vec<PositionActiveModel> = Vec::new();

        for item in params.iter() {
            let mut position = PositionActiveModel {
                ..Default::default()
            };
    
            item.update(&mut position);
            item.update_by_create(&mut position);
            positions.push(position);
        }
        
        let _ = PositionEntity::insert_many(positions).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量职位添加完成"))
    }

    /// 创建部门
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = PositionActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.position_id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.position_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.position_id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.position_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = PositionEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl PositionModel {  
}