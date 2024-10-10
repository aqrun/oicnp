use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Positions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Positions::PositionId)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Positions::Vid).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Positions::Name).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Positions::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(Positions::Status).char_len(1).not_null().default(""))
            .col(ColumnDef::new(Positions::Remark).string_len(500).not_null().default(""))
            .col(ColumnDef::new(Positions::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Positions::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Positions::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Positions::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Positions::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Positions::Table).to_owned()
        ).await
    }
}
