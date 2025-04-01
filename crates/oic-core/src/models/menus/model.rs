use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    models::menus::MenuTreeItem,
    services::menu::build_menu_tree,
    typings::ListData,
    utils::{catch_err, utc_now},
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Set};
use validator::Validate;
use super::{CreateMenuReqParams, MenuFilters, UpdateMenuReqParams, DeleteMenuReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for MenuActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for MenuModel {
    type DataModel = Self;
    type FilterParams = MenuFilters;
    type CreateReqParams = CreateMenuReqParams;
    type UpdateReqParams = UpdateMenuReqParams;
    type DeleteReqParams = DeleteMenuReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = MenuEntity::find()
            .filter(MenuColumn::Id.eq(id))
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
        Ok(Self::default())
    }


    ////
    /// 获取node列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<ListData<Self>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = MenuEntity::find();

        if let Some(x) = params.id {
            q = q.filter(MenuColumn::Id.eq(x));
        }

        if let Some(x) = &params.mid {
            q = q.filter(MenuColumn::Mid.eq(x));
        }

        if let Some(x) = &params.pid {
            q = q.filter(MenuColumn::Pid.eq(x));
        }

        if let Some(x) = &params.depth {
            q = q.filter(MenuColumn::Depth.eq(x));
        }

        if let Some(x) = &params.name {
            if !x.is_empty() {
                q = q.filter(MenuColumn::Name.contains(x));
            }
        }

        let mut order_by = MenuColumn::Id;

        if order_by_str.eq("name") {
            order_by = MenuColumn::Name;
        } else if order_by_str.eq("weight") {
            order_by = MenuColumn::Weight
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

    /// 批量创建 menu
    /// 菜单需要每次添加更新对应的 depth 数据 所以不使用 事务操作
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }

        // 缓存已存在的菜单数据
        let mut exist_menus: HashMap<String, Self> = HashMap::new();

        // 遍历参数列表
        for item in params.iter() {
            // 先使用缓存父菜单数据
            let mut parent_menu: Option<Self> = None;
            let mut pid = String::from("");

            if let Some(x) = &item.pid {
                pid = String::from(x);
            }
            
            if !pid.is_empty() {
                let res = exist_menus.get(pid.as_str());

                if let Some(res) = res {
                    parent_menu = Some(res.clone());
                } else {
                    // 不存在从数据库读取
                    if let Ok(res) = Self::find_by_mid(db, pid.as_str()).await {
                        exist_menus.insert(String::from(res.mid.as_str()), res.clone());
                        parent_menu = Some(res);
                    }
                }
            }

            let mut menu = MenuActiveModel {
                ..Default::default()
            };
    
            item.update(&mut menu);
            item.update_by_create(&mut menu);

            // 不存在父菜单 depth = 1
            if pid.is_empty() {
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
            }

            let menu_model = menu.insert(db).await?;
            let mut menu = menu_model.clone().into_active_model();

            // 深度对应的 p值指定为自身ID
            if menu_model.depth == 1 {
                menu.p1 = Set(menu_model.id);
            }
            if menu_model.depth == 2 {
                menu.p2 = Set(menu_model.id);
            }
            if menu_model.depth == 3 {
                menu.p3 = Set(menu_model.id);
            }
            if menu_model.depth == 4 {
                menu.p4 = Set(menu_model.id);
            }
            if menu_model.depth == 5 {
                menu.p5 = Set(menu_model.id);
            }
            if menu_model.depth == 6 {
                menu.p6 = Set(menu_model.id);
            }
            if menu_model.depth == 7 {
                menu.p7 = Set(menu_model.id);
            }
            if menu_model.depth == 8 {
                menu.p8 = Set(menu_model.id);
            }
            // 更新到数据表
            let menu_model = menu.update(db).await?;
            // 添加缓存数据
            exist_menus.insert(String::from(menu_model.mid.as_str()), menu_model);
        }

        Ok(String::from("批量菜单添加完成"))
    }

    /// 创建 node
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = MenuActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();

        params.update(&mut item);
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = NodeEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl MenuModel {
    ///
    /// 根据MID查找一个
    /// 
    pub async fn find_by_mid(db: &DatabaseConnection, mid: &str) -> ModelResult<Self> {
        if mid.is_empty() {
            return Err(ModelError::Any(format!("mid为空: {}", mid).into()));
        }

        let item = MenuEntity::find()
            .filter(MenuColumn::Mid.eq(mid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在, mid: {}", mid).into())
        })
    }

    ////
    /// 获取node列表树型数据
    /// 
    pub async fn find_tree(db: &DatabaseConnection, params: MenuFilters) -> ModelResult<MenuTreeItem> {
        let order = params.get_order();
        let order_by_str = params.get_order_by();
        let mid = params.mid.unwrap_or(String::from(""));
        
        // 先获取父级数据
        let menu = Self::find_by_mid(db, mid.as_str()).await?;

        // 根据父级查找所有子级元素
        let mut q = MenuEntity::find();

        if menu.depth == 1 {
            q = q.filter(MenuColumn::P1.eq(menu.id));
        }
        if menu.depth == 2 {
            q = q.filter(MenuColumn::P2.eq(menu.id));
        }
        if menu.depth == 3 {
            q = q.filter(MenuColumn::P3.eq(menu.id));
        }
        if menu.depth == 4 {
            q = q.filter(MenuColumn::P4.eq(menu.id));
        }
        if menu.depth == 5 {
            q = q.filter(MenuColumn::P5.eq(menu.id));
        }
        if menu.depth == 6 {
            q = q.filter(MenuColumn::P6.eq(menu.id));
        }
        if menu.depth == 7 {
            q = q.filter(MenuColumn::P7.eq(menu.id));
        }
        if menu.depth == 8 {
            q = q.filter(MenuColumn::P8.eq(menu.id));
        }

        let mut order_by = MenuColumn::Id;

        if order_by_str.eq("name") {
            order_by = MenuColumn::Name;
        } else if order_by_str.eq("weight") {
            order_by = MenuColumn::Weight
        }

        let list = q.order_by(order_by, order).all(db).await?;

        // 列表转为树结构
        let root = build_menu_tree(list);

        Ok(root)
    }
}