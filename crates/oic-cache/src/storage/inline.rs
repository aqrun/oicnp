use crate::metadata::StorageLocation;
use bytes::Bytes;

/// 从内联存储读取数据
/// 
/// 返回 `Bytes` 类型，Clone 是零拷贝的（引用计数）
pub fn read_inline(location: &StorageLocation) -> Option<Bytes> {
    match location {
        StorageLocation::Inline(data) => Some(data.clone()), // 零拷贝 Clone
        StorageLocation::File(_) => None,
    }
}

