use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::types::{MenuId, MenuItem};

/// 主菜单项
pub static MAIN_MENU_ITEMS: Lazy<Vec<MenuItem>> = Lazy::new(|| {
    vec![
        MenuItem { id: MenuId::Home, name: String::from("首页"), href: String::from("/"), vid: String::from("home") },
        MenuItem { id: MenuId::Backend, name: String::from("后端开发"), href: String::from("/category/backend"), vid: String::from("backend") },
        MenuItem { id: MenuId::Frontend, name: String::from("前端开发"), href: String::from("/category/frontend"), vid: String::from("frontend") },
        MenuItem { id: MenuId::Server, name: String::from("服务器"), href: String::from("/category/server"), vid: String::from("server") },
        MenuItem { id: MenuId::Rust, name: String::from("Rust语言"), href: String::from("/category/rust"), vid: String::from("rust") },
        MenuItem { id: MenuId::Diary, name: String::from("随笔"), href: String::from("/category/diary"), vid: String::from("diary") },
    ]
});
