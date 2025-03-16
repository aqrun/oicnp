use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    typings::ListData,
};
use sea_orm::{prelude::*, ActiveValue::NotSet, IntoActiveModel, QueryOrder, Set, TransactionTrait};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use oic_derives::{add_filter_fields, FilterParams};
use validator::Validate;

/// 筛选参数
#[add_filter_fields]
#[derive(Debug, Default, Serialize, Deserialize, FilterParams)]
#[serde(default)]
pub struct RoleFilters {
    pub id: Option<i64>,
    pub vid: Option<String>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for RoleActiveModel {}

impl RoleModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Self> {
        if id <= 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let item = RoleEntity::find()
            .filter(RoleColumn::RoleId.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在,id: {}", id)
        })
    }

    ///
    /// 根据vid查找一个
    /// 
    pub async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> Result<Self> {
        if vid.is_empty() {
            return Err(anyhow!("数据不存在,vid: {}", vid));
        }

        let item = RoleEntity::find()
            .filter(RoleColumn::Vid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在,vid: {}", vid)
        })
    }

    ////
    /// 获取 roles 列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: RoleFilters) -> Result<ListData<Self>> {
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
    pub async fn create(db: &DatabaseConnection, params: &Self) -> Result<Self> {
        let _ = catch_err(params.validate())?;

        let mut item = RoleActiveModel {
            ..Default::default()
        };

        item.set_from_json(json!(params))?;
        item.created_at = Set(Some(utc_now()));
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 批量创建
    pub async fn create_multi(db: &DatabaseConnection, params: &[Self]) -> Result<String> {
        for item in params {
            let _ = catch_err(item.validate())?;
        }
        
        let txn = db.begin().await?;
        let mut list: Vec<RoleActiveModel> = Vec::new();

        for item in params.iter() {
            match RoleActiveModel::from_json(json!(item)) {
                Ok(mut target) => {
                    target.role_id = NotSet;

                    if item.scope.is_none() {
                        target.scope = Set(Some(String::from("")));
                    }

                    if item.remark.is_none() {
                        target.remark = Set(Some(String::from("")));
                    }

                    target.created_at = Set(Some(utc_now()));
                    target.updated_at = Set(Some(utc_now()));
                    
                    list.push(target);
                },
                Err(err) => {
                    txn.rollback().await?;
                    return Err(anyhow!("批量数据有误, {}", err));
                }
            };
        }

        let _ = RoleEntity::insert_many(list).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量角色添加完成"))
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: &Self) -> Result<i64> {
        let _ = catch_err(params.validate())?;
        let id = params.role_id;

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let mut item = Self::find_by_id(&db, id)
            .await?
            .into_active_model();

        if let Some(x) = &params.vid {
            item.vid = Set(Some(String::from(x)));
        }

        if let Some(x) = &params.name {
            item.vid = Set(Some(String::from(x)));
        }

        if let Some(x) = &params.weight {
            item.weight = Set(Some(*x));
        }

        if let Some(x) = &params.status {
            item.status = Set(Some(String::from(x)));
        }

        if let Some(x) = &params.scope {
            item.scope = Set(Some(String::from(x)));
        }

        item.updated_at = Set(Some(utc_now()));
        let item = item.update(db).await?;

        Ok(item.role_id)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<i64> {
        if id <= 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let _res = RoleEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}
