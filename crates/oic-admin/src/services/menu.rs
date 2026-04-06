use anyhow::Result;
use serde::{Deserialize, Serialize};
use oic_core::{
    models::{
        menus::{MenuTreeItem, MenuReqParams},
    },
};
use super::{call_api_with_bearer};
use crate::WebAppContext;

/**
 * 树结构返回的菜单数据
 */
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct MenuRouteItem {
    pub path: String,
    pub component: String,
    pub handle: RouteHandle,
    pub children: Option<Vec<MenuRouteItem>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RouteHandle {
    pub icon: String,
    pub title: String,
    pub order: i32,
    /// 不配置时前端路由会默认放行；因此在序列化为空数组时要省略字段。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    /// 不配置时前端不做按钮级权限过滤；因此序列化为空数组时要省略字段。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
}

pub async fn describe_menu_tree(
    ctx: &WebAppContext,
    bearer: &str,
) -> Result<Vec<MenuTreeItem>> {
    let url = format!("{}/v1/menu/tree", ctx.config.api_url);
    let params = MenuReqParams {
        vid: Some(String::from("backend")),
        ..Default::default()
    };
    let json_value = call_api_with_bearer(&url, bearer, &params).await?;
    let menus_value = json_value
        .pointer("/data/menus")
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]));
    // 从 json_value 中解析出 Vec<MenuTreeItem>   json_value.data.menus
    let menus: Vec<MenuTreeItem> = match serde_json::from_value(menus_value) {
        Ok(menus) => menus,
        Err(_e) => vec![],
    };
    Ok(menus)
}

///
/// 将菜单转换为路由
/// 
pub fn parse_menu_to_routes(menus: Vec<MenuTreeItem>) -> Vec<MenuRouteItem> {
    fn convert_one(menu: MenuTreeItem) -> Vec<MenuRouteItem> {
        // 后端约定：`status == "1"` 表示可用/可见
        if menu.status.trim() != "1" {
            return vec![];
        }

        // 先把字段拆出来，避免 partial move 导致后续 borrow 报错
        let MenuTreeItem {
            path,
            icon,
            label,
            weight,
            children,
            ..
        } = menu;

        // 顶层根节点通常 `path` 为空（例如 vid=backend 的 wrapper），这里把它当作分组节点：
        // 直接返回其 children 作为顶层路由。
        let path = path.trim().to_string();
        if path.is_empty() {
            return children
                .unwrap_or_default()
                .into_iter()
                .flat_map(|c| convert_one(c))
                .collect();
        }

        let children = children.map(parse_menu_to_routes);
        let children = children.filter(|c| !c.is_empty());

        // 叶子节点（没有 children）时，显式指定组件路径，避免前端兜底推导与 layout 规则冲突。
        // 规则：component = path + "/index.tsx"
        let component = if children.is_none() {
            format!("{}/index.tsx", path)
        } else {
            String::new()
        };

        vec![MenuRouteItem {
            path,
            // 如果没有 children 就填充，否则留空让前端按路由结构决定布局/组件。
            component,
            handle: RouteHandle {
                icon,
                title: label,
                order: weight,
                // 目前 MenuTreeItem 不含 roles/permissions；让字段为空并在序列化时跳过，
                // 以便前端 filterTree 在缺省时“默认放行”。
                roles: Vec::new(),
                permissions: Vec::new(),
            },
            children,
        }]
    }

    menus
        .into_iter()
        .flat_map(convert_one)
        .collect()
}

pub async fn get_routes(
    ctx: &WebAppContext,
    bearer: &str,
) -> Result<Vec<MenuRouteItem>> {
    let routes = describe_menu_tree(ctx, bearer).await?;
    // 这里保留一部分内置路由，便于你在菜单为空/未配置时仍能访问主页。
    let mut res = vec![
        MenuRouteItem {
            path: "/home".to_string(),
            component: "/home/index.tsx".to_string(),
            handle: RouteHandle {
                icon: "HomeOutlined".to_string(),
                title: "common.menu.home".to_string(),
                order: 1,
                roles: Vec::new(),
                permissions: Vec::new(),
            },
            children: None,
        },
    ];

    res.extend(parse_menu_to_routes(routes));
    res.push(MenuRouteItem {
        path: "/about".to_string(),
        component: "/about/index.tsx".to_string(),
        handle: RouteHandle {
            icon: "CopyrightOutlined".to_string(),
            title: "common.menu.about".to_string(),
            order: 2,
            roles: Vec::new(),
            permissions: Vec::new(),
        },
        children: None,
    });
    Ok(res)
}