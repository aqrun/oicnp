use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let cms_files_table = Table::create()
            .table(Files::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Files::FileId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Files::Uid).big_integer().not_null().default(0))
            .col(ColumnDef::new(Files::Filename).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Files::Uri).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Files::Storage).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Files::Mime).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Files::Status).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Files::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Files::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Files::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Files::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Files::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(cms_files_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Files::Table).to_owned()
        ).await
    }
}

