use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SideNavItem {
  pub name: String,
  pub vid: String,
  pub href: String,
  pub tags: Option<Vec<String>>,
  pub dynasty: Option<String>,
}

pub static BLOG_CATEGORIES: Lazy<Vec<SideNavItem>> = Lazy::new(|| {
  vec![
    SideNavItem {
      name: "综合".to_string(),
      vid: "all".to_string(),
      href: "/blog/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "Rust语言".to_string(),
      vid: "rust".to_string(),
      href: "/cat/rust/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "服务器".to_string(),
      vid: "server".to_string(),
      href: "/cat/server/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "后端开发".to_string(),
      vid: "backend".to_string(),
      href: "/cat/backend/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "前端开发".to_string(),
      vid: "frontend".to_string(),
      href: "/cat/frontend/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "每日随笔".to_string(),
      vid: "diary".to_string(),
      href: "/cat/diary/".to_string(),
      tags: None,
      dynasty: None,
    },
  ]
});

pub static TOOL_CATEGORIES: Lazy<Vec<SideNavItem>> = Lazy::new(|| {
  vec![
    SideNavItem {
      name: "全部".to_string(),
      vid: "all".to_string(),
      href: "/tool/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "React UI".to_string(),
      vid: "react-ui".to_string(),
      href: "/tool/cat/react-ui/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "静态站点生成器".to_string(),
      vid: "static-site-generator".to_string(),
      href: "/tool/cat/static-site-generator/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "图表 / 可视化".to_string(),
      vid: "charts".to_string(),
      href: "/tool/cat/charts/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "表单与校验".to_string(),
      vid: "forms".to_string(),
      href: "/tool/cat/forms/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "Headless CMS".to_string(),
      vid: "headless-cms".to_string(),
      href: "/tool/cat/headless-cms/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "Markdown / MDX".to_string(),
      vid: "markdown-mdx".to_string(),
      href: "/tool/cat/markdown-mdx/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "AI 应用".to_string(),
      vid: "ai-apps".to_string(),
      href: "/tool/cat/ai-apps/".to_string(),
      tags: None,
      dynasty: None,
    },
  ]
});

pub static POETRY_CATEGORIES: Lazy<Vec<SideNavItem>> = Lazy::new(|| {
  vec![
    SideNavItem {
      name: "全部".to_string(),
      vid: "all".to_string(),
      href: "/poetry/".to_string(),
      tags: None,
      dynasty: None,
    },
    SideNavItem {
      name: "五代诗词".to_string(),
      vid: "wu-dai-shi-ci".to_string(),
      href: "/poetry/cat/wu-dai-shi-ci/".to_string(),
      tags: Some(vec!["花间集".to_string(), "南唐".to_string()]),
      dynasty: Some("五代".to_string()),
    },
    SideNavItem {
      name: "唐诗".to_string(),
      vid: "tang-shi".to_string(),
      href: "/poetry/cat/tang-shi/".to_string(),
      tags: Some(vec!["全唐诗".to_string(), "全宋诗".to_string()]),
      dynasty: Some("唐".to_string()),
    },
    SideNavItem {
      name: "宋词".to_string(),
      vid: "song-ci".to_string(),
      href: "/poetry/cat/song-ci/".to_string(),
      tags: Some(vec!["宋词".to_string()]),
      dynasty: Some("宋".to_string()),
    },
    SideNavItem {
      name: "楚辞".to_string(),
      vid: "chu-ci".to_string(),
      href: "/poetry/cat/chu-ci/".to_string(),
      tags: Some(vec!["楚辞".to_string()]),
      dynasty: Some("先秦".to_string()),
    },
    SideNavItem {
      name: "蒙学".to_string(),
      vid: "meng-xue".to_string(),
      href: "/poetry/cat/meng-xue/".to_string(),
      tags: Some(vec!["蒙学".to_string()]),
      dynasty: None,
    },
    SideNavItem {
      name: "四书五经".to_string(),
      vid: "si-shu-wu-jing".to_string(),
      href: "/poetry/cat/si-shu-wu-jing/".to_string(),
      tags: Some(vec!["四书五经".to_string(), "诗经".to_string(), "论语".to_string()]),
      dynasty: None,
    },
  ]
});
