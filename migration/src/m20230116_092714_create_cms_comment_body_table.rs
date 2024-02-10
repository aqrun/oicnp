use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CommentBody::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CommentBody::CommentId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CommentBody::Body).text().not_null().default(""))
            .col(ColumnDef::new(CommentBody::BodyFormat).string_len(20).not_null().default(""))
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentBody::Table).to_owned())
            .await
    }
}
