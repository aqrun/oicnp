#[cfg(debug_assertions)]
pub const HANDLER_CACHE_TIME: i64 = 1;

/// 生产环境缓存时间 2 小时
#[cfg(not(debug_assertions))]
pub const HANDLER_CACHE_TIME: i64 = 60 * 60 * 2;
