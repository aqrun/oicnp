use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    typings::ListData,
};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Set};
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
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Self> {
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

    ///
    /// 根据MID查找一个
    /// 
    pub async fn find_by_mid(db: &DatabaseConnection, mid: &str) -> Result<Self> {
        if mid.is_empty() {
            return Err(anyhow!("mid为空: {}", mid));
        }

        let item = MenuEntity::find()
            .filter(MenuColumn::Mid.eq(mid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在, mid: {}", mid)
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
    /// 菜单需要每次添加更新对应的 depth 数据 所以不使用 事务操作
    pub async fn create_multi(db: &DatabaseConnection, params: &[CreateMenuReqParams]) -> Result<String> {
        for item in params {
            let _ = catch_err(item.validate())?;
        }

        // 缓存已存在的菜单数据
        let mut exist_menus: HashMap<String, Self> = HashMap::new();

        for item in params.iter() {
            // 先使用缓存父菜单数据
            let mut parent_menu: Option<Self> = None;
            
            if !item.pid.as_str().is_empty() {
                let res = exist_menus.get(item.pid.as_str());

                if let Some(res) = res {
                    parent_menu = Some(res.clone());
                } else {
                    // 不存在从数据库读取
                    match Self::find_by_mid(db, item.pid.as_str()).await {
                        Ok(res) => {
                            exist_menus.insert(String::from(res.mid.as_str()), res.clone());
                            parent_menu = Some(res);
                        },
                        _ => {},
                    };
                }
            }

            match MenuActiveModel::from_json(json!(item)) {
                Ok(mut menu) => {
                    menu.created_at = Set(utc_now());

                    // 不存在父菜单 depth = 1
                    if item.pid.as_str().is_empty() {
                        menu.depth = Set(1);
                    }

                    if let Some(parent_menu) = parent_menu {
                        let depth = parent_menu.depth + 1;
                        menu.depth = Set(depth);
                        menu.p1 = Set(parent_menu.p1);
                        menu.p2 = Set(parent_menu.p2);
                        menu.p3 = Set(parent_menu.p3);
                        menu.p4 = Set(parent_menu.p4);
                        menu.p5 = Set(parent_menu.p5);
                        menu.p6 = Set(parent_menu.p6);
                        menu.p7 = Set(parent_menu.p7);
                        menu.p8 = Set(parent_menu.p8);

                        if depth == 2 {
                            menu.p1 = Set(parent_menu.id);
                        }
                        if depth == 3 {
                            menu.p2 = Set(parent_menu.id);
                        }
                        if depth == 4 {
                            menu.p3 = Set(parent_menu.id);
                        }
                        if depth == 5 {
                            menu.p4 = Set(parent_menu.id);
                        }
                        if depth == 6 {
                            menu.p5 = Set(parent_menu.id);
                        }
                        if depth == 7 {
                            menu.p6 = Set(parent_menu.id);
                        }
                        if depth == 8 {
                            menu.p7 = Set(parent_menu.id);
                        }
                        if depth == 9 {
                            menu.p8 = Set(parent_menu.id);
                        }
                    }

                    let res = menu.insert(db).await?;
                    exist_menus.insert(String::from(res.mid.as_str()), res);
                },
                Err(err) => {
                    return Err(anyhow!("批量数据有误, {}", err));
                }
            };
        }

        Ok(String::from("批量菜单添加完成"))
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateMenuReqParams) -> Result<i32> {
        let _ = catch_err(params.validate())?;
        let id = params.id;

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
    pub async fn delete(db: &DatabaseConnection, params: DeleteMenuReqParams) -> Result<i32> {
        let id = params.id;

        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let _res = NodeEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}