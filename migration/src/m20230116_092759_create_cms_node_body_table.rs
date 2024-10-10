use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(NodeBody::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NodeBody::Nid)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .unique_key()
            )
            .col(ColumnDef::new(NodeBody::Summary).text().not_null().default(""))
            .col(ColumnDef::new(NodeBody::SummaryFormat).string_len(20).not_null().default(""))
            .col(ColumnDef::new(NodeBody::Body).text().not_null().default(""))
            .col(ColumnDef::new(NodeBody::BodyFormat).string_len(20).not_null().default(""))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeBody::Table).to_owned())
            .await
    }
}
