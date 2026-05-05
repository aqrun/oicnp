use crate::models::caches::CacheScope;

///
/// 根据 key 解析缓存作用域
/// 
pub fn parse_cache_scope_by_key(key: &str) -> CacheScope {
    if key.is_empty() {
        return CacheScope::Other;
    }

    if key.starts_with("session-") {
        return CacheScope::Session;
    } else if key.starts_with("captcha-") {
        return CacheScope::Captcha;
    } else if key.starts_with("api:") {
        return CacheScope::Api;
    } else if key.starts_with("web:") {
        return CacheScope::Web;
    } else if key.starts_with("admin:") {
        return CacheScope::Admin;
    }

    CacheScope::Other
}