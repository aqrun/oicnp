use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212131_create_positions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysPositions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysPositions::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysPositions::Vid).string_len(64).default(""))
            .col(ColumnDef::new(SysPositions::Name).string_len(50).default(""))
            .col(ColumnDef::new(SysPositions::Weight).integer().default(0))
            .col(ColumnDef::new(SysPositions::Status).char_len(1).default(""))
            .col(ColumnDef::new(SysPositions::Remark).string_len(500).default(""))
            .col(ColumnDef::new(SysPositions::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysPositions::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysPositions::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysPositions::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysPositions::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysPositions::Table).to_owned()
        ).await
    }
}
