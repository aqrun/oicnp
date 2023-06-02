use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092944_create_cms_configs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsConfigs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsConfigs::Name)
                    .string_len(64)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsConfigs::Data).string_len(512).default(""))
            .col(ColumnDef::new(CmsConfigs::DataType).string_len(32).default(""))
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsConfigs::Table).to_owned())
            .await
    }
}
