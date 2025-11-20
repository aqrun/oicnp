use crate::poetry::create_tables;
use anyhow::Result;

pub async fn init_poetry() -> Result<()> {
    create_tables().await
}
