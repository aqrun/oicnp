use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    utils::catch_err,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder};
use validator::Validate;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use super::{CreatePermissionReqParams, DeletePermissionReqParams, PermissionFilters, UpdatePermissionReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for PermissionActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for PermissionModel {
    type DataModel = Self;
    type FilterParams = PermissionFilters;
    type CreateReqParams = CreatePermissionReqParams;
    type UpdateReqParams = UpdatePermissionReqParams;
    type DeleteReqParams = DeletePermissionReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = PermissionEntity::find()
            .filter(PermissionColumn::PermissionId.eq(id))
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

        let item = PermissionEntity::find()
            .filter(PermissionColumn::Vid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,vid: {}", vid).into())
        })
    }

    ////
    /// 获取 roles 列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        // let order_by_str = params.get_order_by();

        let mut q = PermissionEntity::find();

        if let Some(x) = params.permission_id {
            if x > 0 {
                q = q.filter(PermissionColumn::PermissionId.eq(x));
            }
        }

        if let Some(x) = &params.vid {
            if !x.is_empty() {
                q = q.filter(PermissionColumn::Vid.contains(x.as_str()));
            }
        }

        let order_by = PermissionColumn::Weight;

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
        let mut exist_permissions: HashMap<String, Self> = HashMap::new();

        // 遍历参数列表
        for item in params.iter() {
            // 先使用缓存父菜单数据
            let mut parent_permission: Option<Self> = None;
            let mut parent_vid = String::from("");

            if let Some(x) = &item.parent_vid {
                parent_vid = String::from(x);
            }
            
            if !parent_vid.is_empty() {
                let res = exist_permissions.get(parent_vid.as_str());

                if let Some(res) = res {
                    parent_permission = Some(res.clone());
                } else {
                    // 不存在从数据库读取
                    if let Ok(res) = Self::find_by_vid(db, parent_vid.as_str()).await {
                        exist_permissions.insert(String::from(res.vid.as_str()), res.clone());
                        parent_permission = Some(res);
                    }
                }
            }

            let mut permission = PermissionActiveModel {
                ..Default::default()
            };
    
            item.update(&mut permission);
            item.update_by_create(&mut permission);

            if let Some(parent_permission) = parent_permission {
                permission.pid = Set(parent_permission.permission_id);
            }

            let permission_model = permission.insert(db).await?;
            // let permission = permission_model.clone().into_active_model();

            // // 更新到数据表
            // let permission_model = permission.update(db).await?;
            // 添加缓存数据
            exist_permissions.insert(String::from(permission_model.vid.as_str()), permission_model);
        }

        Ok(String::from("批量权限添加完成"))
    }

    /// 创建
     async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = PermissionActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.permission_id)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.permission_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);

        let item = item.update(db).await?;

        Ok(item.permission_id)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.permission_id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = PermissionEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}

impl PermissionModel {
}
