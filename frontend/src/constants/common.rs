use crate::types::{MenuId, MenuItem};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub const SITE_NAME: &'static str = "爱喜";

/// 主菜单项
pub static MAIN_MENU_ITEMS: Lazy<Vec<MenuItem>> = Lazy::new(|| {
    vec![
        MenuItem {
            id: MenuId::Home,
            name: "首页",
            href: "/",
            vid: MenuId::Home.get_vid(),
        },
        MenuItem {
            id: MenuId::Backend,
            name: "后端开发",
            href: "/category/backend",
            vid: MenuId::Backend.get_vid(),
        },
        MenuItem {
            id: MenuId::Frontend,
            name: "前端开发",
            href: "/category/frontend",
            vid: MenuId::Backend.get_vid(),
        },
        MenuItem {
            id: MenuId::Server,
            name: "服务器",
            href: "/category/server",
            vid: MenuId::Backend.get_vid(),
        },
        MenuItem {
            id: MenuId::Rust,
            name: "Rust语言",
            href: "/category/rust",
            vid: MenuId::Backend.get_vid(),
        },
        MenuItem {
            id: MenuId::Diary,
            name: "随笔",
            href: "/category/diary",
            vid: MenuId::Backend.get_vid(),
        },
    ]
});
