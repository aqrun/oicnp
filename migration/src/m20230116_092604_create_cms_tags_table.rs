use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Tags::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Tags::TagId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Tags::TagVid).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Tags::TagName).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Tags::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(Tags::TagCount).big_integer().not_null().default(0))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await
    }
}
