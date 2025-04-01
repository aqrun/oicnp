use crate::models::Category;
use oic_core::prelude::once_cell::sync::Lazy;

/// 阅读 VID
pub const VID_READING: &str = "reading";

/// 内容分类
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
            dir: "src/content/backend",
            weight: 10,
            parent: "cms",
        },
        Category {
            name: "前端开发",
            vid: "frontend",
            dir: "src/content/frontend",
            weight: 20,
            parent: "cms",
        },
        Category {
            name: "Rust 语言",
            vid: "rust",
            dir: "src/content/rust",
            weight: 30,
            parent: "cms",
        },
        Category {
            name: "服务器",
            vid: "server",
            dir: "src/content/server",
            weight: 40,
            parent: "cms",
        },
        Category {
            name: "每日随笔",
            vid: "diary",
            dir: "src/content/diary",
            weight: 50,
            parent: "cms",
        },
        Category {
            name: "阅读小记",
            vid: VID_READING,
            dir: "src/content/reading",
            weight: 50,
            parent: "cms",
        },
    ]
});
