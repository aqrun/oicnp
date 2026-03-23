use crate::error::{CacheError, Result};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::path::{Path, PathBuf};

pub const INDEX_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("cache_index");
pub const VALUE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("cache_value");

fn db_path(base_path: &Path) -> PathBuf {
    base_path.join("cache.redb")
}

pub fn open_db(base_path: &Path) -> Result<Database> {
    let path = db_path(base_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(CacheError::Io)?;
    }
    Database::create(path).map_err(|e| CacheError::InvalidConfig(format!("Failed to open redb: {}", e)))
}

pub fn read_value(base_path: &Path, key: &[u8]) -> Result<Option<Vec<u8>>> {
    let db = open_db(base_path)?;
    let read_txn = db
        .begin_read()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to begin read tx: {}", e)))?;
    let table = match read_txn.open_table(VALUE_TABLE) {
        Ok(table) => table,
        Err(_) => return Ok(None),
    };
    let value = table
        .get(key)
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to read value: {}", e)))?
        .map(|v| v.value().to_vec());
    Ok(value)
}

pub fn write_value(base_path: &Path, key: &[u8], value: &[u8]) -> Result<()> {
    let db = open_db(base_path)?;
    let write_txn = db
        .begin_write()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to begin write tx: {}", e)))?;
    {
        let mut table = write_txn
            .open_table(VALUE_TABLE)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to open value table: {}", e)))?;
        table
            .insert(key, value)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to write value: {}", e)))?;
    }
    write_txn
        .commit()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to commit write tx: {}", e)))?;
    Ok(())
}

pub fn delete_value(base_path: &Path, key: &[u8]) -> Result<()> {
    let db = open_db(base_path)?;
    let write_txn = db
        .begin_write()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to begin write tx: {}", e)))?;
    {
        let mut table = write_txn
            .open_table(VALUE_TABLE)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to open value table: {}", e)))?;
        let _ = table
            .remove(key)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to delete value: {}", e)))?;
    }
    write_txn
        .commit()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to commit delete tx: {}", e)))?;
    Ok(())
}

pub fn replace_all_index(base_path: &Path, entries: &[(String, Vec<u8>)]) -> Result<()> {
    let db = open_db(base_path)?;
    let write_txn = db
        .begin_write()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to begin write tx: {}", e)))?;
    {
        let mut table = write_txn
            .open_table(INDEX_TABLE)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to open index table: {}", e)))?;
        let existing_keys: Vec<Vec<u8>> = table
            .iter()
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to iterate index table: {}", e)))?
            .map(|item| {
                item.map(|(k, _)| k.value().to_vec())
                    .map_err(|e| CacheError::InvalidConfig(format!("Failed to read index row: {}", e)))
            })
            .collect::<Result<Vec<_>>>()?;
        for key in existing_keys {
            let _ = table
                .remove(key.as_slice())
                .map_err(|e| CacheError::InvalidConfig(format!("Failed to clear index row: {}", e)))?;
        }
        for (key, value) in entries {
            table
                .insert(key.as_bytes(), value.as_slice())
                .map_err(|e| CacheError::InvalidConfig(format!("Failed to write index entry: {}", e)))?;
        }
    }
    write_txn
        .commit()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to commit index tx: {}", e)))?;
    Ok(())
}

pub fn load_all_index(base_path: &Path) -> Result<Vec<(String, Vec<u8>)>> {
    let path = db_path(base_path);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let db = open_db(base_path)?;
    let read_txn = db
        .begin_read()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to begin read tx: {}", e)))?;
    let table = match read_txn.open_table(INDEX_TABLE) {
        Ok(table) => table,
        Err(_) => return Ok(Vec::new()),
    };

    let mut out = Vec::new();
    let iter = table
        .iter()
        .map_err(|e| CacheError::InvalidConfig(format!("Failed to iterate index table: {}", e)))?;

    for item in iter {
        let (k, v) = item
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to read index row: {}", e)))?;
        let key = String::from_utf8(k.value().to_vec())
            .map_err(|e| CacheError::Serialization(format!("Invalid UTF-8 key in redb index: {}", e)))?;
        out.push((key, v.value().to_vec()));
    }
    Ok(out)
}
