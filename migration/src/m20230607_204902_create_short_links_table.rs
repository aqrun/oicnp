use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230607_204902_create_short_links_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsShortLinks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsShortLinks::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsShortLinks::Link).string_len(512).default(""))
            .col(ColumnDef::new(CmsShortLinks::Name).string_len(255).default(""))
            .col(ColumnDef::new(CmsShortLinks::Description).string_len(512).default(""))
            .col(ColumnDef::new(CmsShortLinks::Viewed).integer().default(0))
            .col(ColumnDef::new(CmsShortLinks::Deleted).char_len(1).default("0"))
            .col(ColumnDef::new(CmsShortLinks::CreatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(CmsShortLinks::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsShortLinks::Table).to_owned())
            .await
    }
}
