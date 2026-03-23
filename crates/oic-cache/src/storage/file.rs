use crate::error::{CacheError, Result};
use crate::metadata::StorageLocation;
use crate::storage::redb::{delete_value, read_value, write_value};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use bytes::Bytes;

/// 生成文件路径
pub fn generate_file_path(base_path: &Path, key: &str) -> PathBuf {
    // 使用 key 的哈希值来生成文件路径，避免文件名冲突
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let hash = hasher.finalize();
    let hash_str = hex::encode(&hash[..8]); // 使用前8字节作为文件名
    
    // 创建两级目录结构以分散文件
    let dir1 = &hash_str[0..2];
    let dir2 = &hash_str[2..4];
    
    base_path.join(dir1).join(dir2).join(format!("{}.cache", hash_str))
}

/// 从文件存储读取数据
/// 
/// 返回 `Bytes` 类型，Clone 是零拷贝的（引用计数）
pub async fn read_file(location: &StorageLocation, base_path: &Path) -> Result<Option<Bytes>> {
    match location {
        StorageLocation::Inline(_) => Ok(None),
        StorageLocation::File(file_path) => {
            let key = file_path.as_bytes().to_vec();
            let base_path = base_path.to_path_buf();
            tokio::task::spawn_blocking(move || {
                read_value(&base_path, &key).map(|opt| opt.map(Bytes::from))
            })
            .await
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to join read task: {}", e)))?
        }
    }
}

/// 写入文件存储
pub async fn write_file(
    base_path: &Path,
    key: &str,
    data: &[u8],
) -> Result<String> {
    let file_path = generate_file_path(base_path, key)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("{}.cache", key));
    let table_key = file_path.as_bytes().to_vec();
    let value = data.to_vec();
    let base_path = base_path.to_path_buf();
    tokio::task::spawn_blocking(move || write_value(&base_path, &table_key, &value))
        .await
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to join write task: {}", e)))??;
    Ok(file_path)
}

/// 删除文件
pub async fn delete_file(base_path: &Path, file_path: &str) -> Result<()> {
    let key = file_path.as_bytes().to_vec();
    let base_path = base_path.to_path_buf();
    tokio::task::spawn_blocking(move || delete_value(&base_path, &key))
        .await
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to join delete task: {}", e)))??;
    Ok(())
}

