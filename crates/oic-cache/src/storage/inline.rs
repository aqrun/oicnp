use crate::metadata::StorageLocation;

/// 从内联存储读取数据
pub fn read_inline(location: &StorageLocation) -> Option<Vec<u8>> {
    match location {
        StorageLocation::Inline(data) => Some(data.clone()),
        StorageLocation::File(_) => None,
    }
}

