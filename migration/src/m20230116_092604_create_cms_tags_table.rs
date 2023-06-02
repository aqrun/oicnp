use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092604_create_cms_tags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsTags::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsTags::TagId)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsTags::Vid).string_len(255).default(""))
            .col(ColumnDef::new(CmsTags::Name).string_len(128).default(""))
            .col(ColumnDef::new(CmsTags::Weight).integer().default(0))
            .col(ColumnDef::new(CmsTags::Count).big_integer().default(0))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsTags::Table).to_owned())
            .await
    }
}
