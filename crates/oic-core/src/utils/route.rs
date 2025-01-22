use crate::constants::API_PREFIX;

///
/// 生成接口前缀
/// 
pub fn get_api_prefix(version: &str, prefix: &str) -> String {
    let mut parts: Vec<&str> = vec!();

    if !version.is_empty() {
        parts.push(version);
    }

    if !API_PREFIX.is_empty() {
        parts.push(API_PREFIX);
    }

    if !prefix.is_empty() {
        parts.push(prefix);
    }

    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_auth() {
        assert_eq!(get_api_prefix("v1", "auth"), "v1/auth");
    }

    #[test]
    fn test_index() {
        assert_eq!(get_api_prefix("", ""), "");
    }
}