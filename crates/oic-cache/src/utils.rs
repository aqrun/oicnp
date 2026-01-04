use crate::metadata::ContentInfo;
use sha2::{Sha256, Digest};

/// 生成 ETag
pub fn generate_etag(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    format!("\"{}\"", hex::encode(&hash[..16]))
}

/// 创建 ContentInfo
pub fn create_content_info(content_type: String, data: &[u8]) -> ContentInfo {
    ContentInfo {
        content_type,
        etag: generate_etag(data),
        encoding: None,
        charset: None,
    }
}

