use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ShortLinks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ShortLinks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(ShortLinks::Link).string_len(512).not_null().default(""))
            .col(ColumnDef::new(ShortLinks::Name).string_len(255).not_null().default(""))
            .col(ColumnDef::new(ShortLinks::Description).string_len(512).not_null().default(""))
            .col(ColumnDef::new(ShortLinks::Viewed).integer().not_null().default(0))
            .col(ColumnDef::new(ShortLinks::Deleted).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(ShortLinks::CreatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(ShortLinks::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ShortLinks::Table).to_owned())
            .await
    }
}
