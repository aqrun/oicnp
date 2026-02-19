mod api;
mod cache;
mod base;
mod html;

pub use api::*;
pub use cache::get_cached_or_render;
// CacheConfig 目前主要在 app.rs 中使用，从 oic_cache 导入
// 如果 Controller 需要使用，可以从 cache 模块导入
pub use cache::CacheConfig;
pub use base::*;
pub use html::*;