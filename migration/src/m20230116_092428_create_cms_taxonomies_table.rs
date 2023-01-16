use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092428_create_cms_taxonomies_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsTaxonomies::Table)
            .if_not_exits()
            .col(
                ColumnDef::new(CmsTaxonomies::Tid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsTaxonomies::Vid).string_len(255).default(""))
            .col(ColumnDef::new(CmsTaxonomies::Pid).string_len(32).default(""))
            .col(ColumnDef::new(CmsTaxonomies::Name).string_len(128).default(""))
            .col(ColumnDef::new(CmsTaxonomies::Description).string_len(512).default(""))
            .col(ColumnDef::new(CmsTaxonomies::DescriptionFormat).string_len(20).default(""))
            .col(ColumnDef::new(CmsTaxonomies::Weight).integer().default(0))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsTaxonomies::Table).to_owned())
            .await
    }
}
