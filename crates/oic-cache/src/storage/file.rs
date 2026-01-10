use crate::error::{CacheError, Result};
use crate::metadata::StorageLocation;
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
            let full_path = if Path::new(file_path).is_absolute() {
                PathBuf::from(file_path)
            } else {
                base_path.join(file_path)
            };
            
            tokio::fs::read(&full_path)
                .await
                .map(|data| Some(Bytes::from(data))) // 零成本转换：Vec<u8> -> Bytes
                .map_err(|e| CacheError::Io(e))
        }
    }
}

/// 写入文件存储
pub async fn write_file(
    base_path: &Path,
    key: &str,
    data: &[u8],
) -> Result<String> {
    let file_path = generate_file_path(base_path, key);
    
    // 确保目录存在
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| CacheError::Io(e))?;
    }
    
    // 写入文件
    tokio::fs::write(&file_path, data)
        .await
        .map_err(|e| CacheError::Io(e))?;
    
    // 返回相对路径
    Ok(file_path
        .strip_prefix(base_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| file_path.to_string_lossy().to_string()))
}

/// 删除文件
pub async fn delete_file(base_path: &Path, file_path: &str) -> Result<()> {
    let full_path = if Path::new(file_path).is_absolute() {
        PathBuf::from(file_path)
    } else {
        base_path.join(file_path)
    };
    
    if full_path.exists() {
        tokio::fs::remove_file(&full_path)
            .await
            .map_err(|e| CacheError::Io(e))?;
    }
    
    Ok(())
}

