use crate::models::Category;
use oicnp_core::prelude::once_cell::sync::Lazy;

pub static CATEGORIES: Lazy<Vec<Category<'static>>> = Lazy::new(|| {
    vec![
        Category {
            name: "内容管理",
            vid: "cms",
            dir: "",
            weight: 1,
            parent: "",
        },
        Category {
            name: "后端开发",
            vid: "backend",
            dir: "content/backend",
            weight: 10,
            parent: "cms",
        },
        Category {
            name: "前端开发",
            vid: "frontend",
            dir: "content/frontend",
            weight: 20,
            parent: "cms",
        },
        Category {
            name: "Rust 语言",
            vid: "rust",
            dir: "content/rust",
            weight: 30,
            parent: "cms",
        },
        Category {
            name: "服务器",
            vid: "server",
            dir: "content/server",
            weight: 40,
            parent: "cms",
        },
        Category {
            name: "每日随笔",
            vid: "diary",
            dir: "content/diary",
            weight: 50,
            parent: "cms",
        },
    ]
});
