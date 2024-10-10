use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UserPositionMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserPositionMap::Uid)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserPositionMap::PositionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserPositionMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(UserPositionMap::Uid)
                    .col(UserPositionMap::PositionId)
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(UserPositionMap::Table).to_owned()
        ).await
    }
}