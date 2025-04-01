use crate::{
    entities::prelude::*,
    utils::catch_err,
    typings::ListData,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait};
use validator::Validate;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use super::{CreateRoleReqParams, DeleteRoleReqParams, RoleFilters, UpdateRoleReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for RoleActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for RoleModel {
    type CreateReqParams = CreateRoleReqParams;

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }
        
        let txn = db.begin().await?;
        let mut list: Vec<RoleActiveModel> = Vec::new();

        for item in params.iter() {
            let mut role = RoleActiveModel {
                ..Default::default()
            };

            item.update(&mut role);
            item.update_by_create(&mut role);

            list.push(role);
        }

        let _ = RoleEntity::insert_many(list).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量角色添加完成"))
    }
}

impl RoleModel {
    pub async fn find_by_user(db: &DatabaseConnection, user: &UserModel) -> ModelResult<Self> {
        let role = RoleEntity::find()
            .inner_join(UserRoleMapEntity)
            .filter(UserRoleMapColumn::Uid.eq(user.uid))
            .one(db)
            .await?;
        role.ok_or_else(|| ModelError::EntityNotFound)
    }

    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = RoleEntity::find()
            .filter(RoleColumn::RoleId.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据vid查找一个
    /// 
    pub async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
        if vid.is_empty() {
            return Err(ModelError::Any(format!("数据不存在,vid: {}", vid).into()));
        }

        let item = RoleEntity::find()
            .filter(RoleColumn::Vid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,vid: {}", vid).into())
        })
    }

    ////
    /// 获取 roles 列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: RoleFilters) -> ModelResult<ListData<Self>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        // let order_by_str = params.get_order_by();

        let mut q = RoleEntity::find();

        if let Some(x) = params.id {
            if x > 0 {
                q = q.filter(RoleColumn::RoleId.eq(x));
            }
        }

        if let Some(x) = params.vid {
            if !x.is_empty() {
                q = q.filter(RoleColumn::Vid.contains(x.as_str()));
            }
        }

        let order_by = RoleColumn::Weight;

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        let res = ListData {
            data: list,
            page,
            page_size,
            total,
        };

        Ok(res)
    }

    /// 创建
    pub async fn create(db: &DatabaseConnection, params: &CreateRoleReqParams) -> ModelResult<Self> {
        catch_err(params.validate())?;

        let mut item = RoleActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: &UpdateRoleReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.role_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);

        let item = item.update(db).await?;

        Ok(item.role_id)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: &DeleteRoleReqParams) -> ModelResult<i64> {
        let id = params.role_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = RoleEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}
