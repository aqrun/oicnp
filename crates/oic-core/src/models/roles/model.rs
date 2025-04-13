use crate::{
    entities::prelude::*, models::permissions::CreatePermissionReqParams, typings::ListData, utils::catch_err
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
    type DataModel = Self;
    type FilterParams = RoleFilters;
    type CreateReqParams = CreateRoleReqParams;
    type UpdateReqParams = UpdateRoleReqParams;
    type DeleteReqParams = DeleteRoleReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
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
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
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
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<ListData<Self>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        // let order_by_str = params.get_order_by();

        let mut q = RoleEntity::find();

        if let Some(x) = params.role_id {
            if x > 0 {
                q = q.filter(RoleColumn::RoleId.eq(x));
            }
        }

        if let Some(x) = &params.vid {
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

        // 为角色指定权限
        Self::assign_multi_role_permissions(db, params).await?;

        Ok(String::from("批量角色添加完成"))
    }

    /// 创建
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = RoleActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.role_id)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
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
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
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
    /// 根据 permission_vids 给角色指定权限
    /// 
    pub async fn assign_multi_role_permissions(
        db: &DatabaseConnection,
        params: &[CreateRoleReqParams],
    ) -> ModelResult<()> {
        // 需要批量创建的 user role 关联关系
        let mut role_permission_list: Vec<RolePermissionsMapActiveModel> = Vec::new();

        let all_permissions = PermissionEntity::find()
            .all(db)
            .await?;
        let all_roles = RoleEntity::find()
            .all(db)
            .await?;

        for item in params.iter() {
            let mut role_vid = String::from("");
            let mut permission_vids: Vec<String> = Vec::new();

            if let Some(x) = &item.vid {
                role_vid = String::from(x);
            } else {
                continue;
            }

            if let Some(x) = &item.permission_vids {
                permission_vids = x.clone();
            }

            let role = match all_roles.iter().find(|item| {
                item.vid.eq(role_vid.as_str())
            }) {
                Some(x) => x,
                _ => {
                    continue;
                }
            };

            for permission_vid in permission_vids {
                let permission = match all_permissions.iter().find(|item| {
                    item.vid.eq(permission_vid.as_str())
                }) {
                    Some(x) => x,
                    _ => continue,
                };

                let role_permission_map = RolePermissionsMapActiveModel {
                    role_id: Set(role.role_id),
                    permission_id: Set(permission.permission_id),
                    ..Default::default()
                };
                role_permission_list.push(role_permission_map);
            }
        }

        let _ = RolePermissionsMapEntity::insert_many(role_permission_list).exec(db).await?;

        Ok(())
    }
}
