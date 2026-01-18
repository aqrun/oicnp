
/// API 前缀
pub static API_PREFIX: &str = "";
/// 日期格式
pub static DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub static DATE_FORMAT: &str = "%Y-%m-%d";

/// 正常登陆过期时间 24小时
pub const LOGIN_EXPIRE_TIME: u64 = 60 * 60 * 24;
/// 记住登陆过期时间 7 days
pub const LOGIN_REMEMBER_EXPIRE_TIME: u64 = 60 * 60 * 24 * 7;