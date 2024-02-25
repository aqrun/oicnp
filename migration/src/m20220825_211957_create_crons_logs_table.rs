use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CronLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CronLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(CronLogs::CronId).string_len(32).not_null())
            .col(ColumnDef::new(CronLogs::LotId).string_len(32).not_null().default(""))
            .col(ColumnDef::new(CronLogs::Weight).small_integer().not_null().default(0))
            .col(ColumnDef::new(CronLogs::Name).string_len(64).not_null())
            .col(ColumnDef::new(CronLogs::Group).string_len(64).not_null().default(""))
            .col(ColumnDef::new(CronLogs::InvokeTarget).string_len(500).not_null().default(""))
            .col(ColumnDef::new(CronLogs::Params).string_len(500).not_null().default(""))
            .col(ColumnDef::new(CronLogs::Message).string_len(500).not_null().default(""))
            .col(ColumnDef::new(CronLogs::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(CronLogs::ExceptionInfo).string_len(2000).not_null().default(""))
            .col(ColumnDef::new(CronLogs::IsOnce).char_len(1).not_null().default(""))
            .col(
                ColumnDef::new(CronLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(CronLogs::ElapsedTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(CronLogs::Table).to_owned()
        ).await
    }
}

