
/// 获取URL链接
pub fn get_url(uri: &str) -> String {
    let base_url = "/";

    let mut url = format!("{}{}", base_url, uri);

    url.replace("//", "/")
}