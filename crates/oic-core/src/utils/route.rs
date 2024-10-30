use crate::constants::API_ADMIN_PREFIX;

///
/// 生成admin接口前缀
/// 
pub fn get_admin_prefix(prefix: &str) -> String {
  format!("{API_ADMIN_PREFIX}/{prefix}")
}