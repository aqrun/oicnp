use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(NodeCommentsMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(NodeCommentsMap::Bundle).string_len(20).not_null().default(""))
            .col(
                ColumnDef::new(NodeCommentsMap::Nid)
                .big_integer()
                .not_null()
                .default(0)
            )
            .col(
                ColumnDef::new(NodeCommentsMap::CommentId)
                .big_integer()
                .not_null()
                .default(0)
            )
            .primary_key(
                Index::create()
                    .col(NodeCommentsMap::Nid)
                    .col(NodeCommentsMap::CommentId),
            )
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeCommentsMap::Table).to_owned())
            .await
    }
}
