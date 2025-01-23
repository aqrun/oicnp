use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    typings::ListData,
};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Set, TransactionTrait};
use serde_json::json;
use validator::Validate;
use super::{CreateMenuReqParams, MenuFilters, UpdateMenuReqParams, DeleteMenuReqParams};
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
impl ActiveModelBehavior for MenuActiveModel {}

impl MenuModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Self> {
        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let item = MenuEntity::find()
            .filter(MenuColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在,id: {}", id)
        })
    }

    ////
    /// 获取node列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: MenuFilters) -> Result<ListData<Self>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = MenuEntity::find();

        if let Some(x) = params.id {
            if x > 0 {
                q = q.filter(MenuColumn::Id.eq(x));
            }
        }

        if let Some(x) = params.title {
            if !x.is_empty() {
                q = q.filter(MenuColumn::Name.contains(&x));
            }
        }

        let mut order_by = MenuColumn::Id;

        if order_by_str.eq("title") {
            order_by = MenuColumn::Name;
        }

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

    /// 创建 node
    pub async fn create(db: &DatabaseConnection, params: &CreateMenuReqParams) -> Result<Self> {
        let _ = catch_err(params.validate())?;

        let mut item = MenuActiveModel {
            ..Default::default()
        };

        item.set_from_json(json!(params))?;
        item.created_at = Set(utc_now());
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 批量创建 node
    pub async fn create_multi(db: &DatabaseConnection, params: &[CreateMenuReqParams]) -> Result<String> {
        let _ = catch_err(params.validate())?;
        
        let txn = db.begin().await?;
        let mut notes: Vec<MenuActiveModel> = Vec::new();

        for item in params.iter() {
            match MenuActiveModel::from_json(json!(item)) {
                Ok(mut note) => {
                    note.created_at = Set(utc_now());
                    notes.push(note);
                },
                Err(err) => {
                    txn.rollback().await?;
                    return Err(anyhow!("批量数据有误, {}", err));
                }
            };
        }
        
        let _ = MenuEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量node添加完成"))
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateMenuReqParams) -> Result<i32> {
        let _ = catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let mut item = Self::find_by_id(&db, id)
            .await?
            .into_active_model();

        item.set_from_json(json!(params))?;
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.update(db).await?;

        Ok(item.id)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteMenuReqParams) -> Result<i64> {
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let _res = NodeEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}