use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212300_create_user_position_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUserPositionMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUserPositionMap::Uid)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysUserPositionMap::PositionId)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysUserPositionMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(SysUserPositionMap::Uid)
                    .col(SysUserPositionMap::PositionId)
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUserPositionMap::Table).to_owned()
        ).await
    }
}