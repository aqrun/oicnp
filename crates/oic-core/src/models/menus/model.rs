use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    models::menus::MenuTreeItem,
    services::menu::build_menu_tree,
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
    /// 根据VID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
        if vid.is_empty() {
            return Err(ModelError::Any(format!("vid为空: {}", vid).into()));
        }

        let item = MenuEntity::find()
            .filter(MenuColumn::Vid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在, vid: {}", vid).into())
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

        let mut q = MenuEntity::find();

        if let Some(x) = params.id {
            q = q.filter(MenuColumn::Id.eq(x));
        }

        if let Some(x) = &params.vid {
            q = q.filter(MenuColumn::Vid.eq(x));
        }

        if let Some(x) = &params.pid {
            q = q.filter(MenuColumn::Pid.eq(*x));
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

        Ok((list, total))
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
            let mut parent_vid = String::from("");

            if let Some(x) = &item.parent_vid {
                parent_vid = String::from(x);
            }
            
            if !parent_vid.is_empty() {
                let res = exist_menus.get(parent_vid.as_str());

                if let Some(res) = res {
                    parent_menu = Some(res.clone());
                } else {
                    // 不存在从数据库读取
                    if let Ok(res) = Self::find_by_vid(db, parent_vid.as_str()).await {
                        exist_menus.insert(String::from(res.vid.as_str()), res.clone());
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
            if parent_vid.is_empty() {
                menu.depth = Set(1);
            }

            if let Some(parent_menu) = parent_menu {
                let depth = parent_menu.depth + 1;
                menu.pid = Set(parent_menu.id);
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
            exist_menus.insert(String::from(menu_model.vid.as_str()), menu_model);
        }

        Self::assign_multi_menu_permissions(db, params).await?;

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
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.update(db).await?;

        Ok(item.id)
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

        Ok(id)
    }
}

impl MenuModel {
    ////
    /// 获取node列表树型数据
    /// 
    pub async fn find_tree(db: &DatabaseConnection, params: MenuFilters) -> ModelResult<MenuTreeItem> {
        let order = params.get_order();
        let order_by_str = params.get_order_by();
        let vid = params.vid.unwrap_or(String::from(""));
        
        // 先获取父级数据
        let menu = Self::find_by_vid(db, vid.as_str()).await?;

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

    ///
    /// 根据 permission_vids 给菜单指定权限
    /// 
    pub async fn assign_multi_menu_permissions(
        db: &DatabaseConnection,
        params: &[CreateMenuReqParams],
    ) -> ModelResult<()> {
        // 需要批量创建的 menu permission 关联关系
        let mut menu_permissions_list: Vec<MenuPermissionsMapActiveModel> = Vec::new();

        let all_permissions = PermissionEntity::find()
            .all(db)
            .await?;
        let all_menus = MenuEntity::find()
            .all(db)
            .await?;

        for item in params.iter() {
            let mut menu_vid = String::from("");
            let mut permission_vids: Vec<String> = Vec::new();

            if let Some(x) = &item.vid {
                menu_vid = String::from(x);
            } else {
                continue;
            }

            if let Some(x) = &item.permission_vids {
                permission_vids = x.clone();
            }

            let menu = match all_menus.iter().find(|item| {
                item.vid.eq(menu_vid.as_str())
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

                let menu_permission_map = MenuPermissionsMapActiveModel {
                    menu_id: Set(menu.id),
                    permission_id: Set(permission.permission_id),
                    ..Default::default()
                };
                menu_permissions_list.push(menu_permission_map);
            }
        }

        let _ = MenuPermissionsMapEntity::insert_many(menu_permissions_list).exec(db).await?;

        Ok(())
    }
}