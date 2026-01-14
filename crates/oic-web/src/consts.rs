#[cfg(debug_assertions)]
pub const HANDLER_CACHE_TIME: i64 = 1;

#[cfg(not(debug_assertions))]
pub const HANDLER_CACHE_TIME: i64 = 60 * 60 * 24;