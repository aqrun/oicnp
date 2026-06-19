use chrono::Utc;

use crate::uuid;

/// 生成相对对象 key，格式：YYYY/MM/{uuid}.ext
pub fn build_object_key(filename: &str) -> String {
    let date_path = Utc::now().format("%Y/%m").to_string();
    let ext = filename.split('.').last().unwrap_or("");
    let real_file_name = format!("{}.{}", uuid!(), ext);
    format!("{}/{}", date_path, real_file_name)
}

pub fn join_public_url(base_uri: &str, prefix: &str, uri: &str) -> String {
    let base = base_uri.trim_end_matches('/');
    let uri = uri.trim_start_matches('/');

    if prefix.is_empty() {
        format!("{}/{}", base, uri)
    } else {
        let prefix = prefix.trim_matches('/');
        format!("{}/{}/{}", base, prefix, uri)
    }
}
