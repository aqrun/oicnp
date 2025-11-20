use crate::poetry;
use anyhow::Result;

pub async fn init_poetry() -> Result<()> {
    poetry::create_tables().await
}

pub async fn sync_data() -> Result<()> {
    poetry::sync_data().await
}