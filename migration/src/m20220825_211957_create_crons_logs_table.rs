use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211957_create_crons_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysCronLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysCronLogs::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysCronLogs::CronId).string_len(32).not_null())
            .col(ColumnDef::new(SysCronLogs::LotId).string_len(32).default(""))
            .col(ColumnDef::new(SysCronLogs::Weight).small_integer().default(0))
            .col(ColumnDef::new(SysCronLogs::Name).string_len(64).not_null())
            .col(ColumnDef::new(SysCronLogs::Group).string_len(64).default(""))
            .col(ColumnDef::new(SysCronLogs::InvokeTarget).string_len(500).default(""))
            .col(ColumnDef::new(SysCronLogs::Params).string_len(500).default(""))
            .col(ColumnDef::new(SysCronLogs::Message).string_len(500).default(""))
            .col(ColumnDef::new(SysCronLogs::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysCronLogs::ExceptionInfo).string_len(2000).default(""))
            .col(ColumnDef::new(SysCronLogs::IsOnce).char_len(1).default(""))
            .col(
                ColumnDef::new(SysCronLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysCronLogs::ElapsedTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysCronLogs::Table).to_owned()
        ).await
    }
}

