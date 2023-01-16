use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212228_create_update_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUpdateLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUpdateLogs::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysUpdateLogs::AppVersion).string_len(10).default(""))
            .col(ColumnDef::new(SysUpdateLogs::BackendVersion).string_len(10).default(""))
            .col(ColumnDef::new(SysUpdateLogs::Title).string_len(100).default(""))
            .col(ColumnDef::new(SysUpdateLogs::Content).text().default(""))
            .col(ColumnDef::new(SysUpdateLogs::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysUpdateLogs::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysUpdateLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysUpdateLogs::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysUpdateLogs::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUpdateLogs::Table).to_owned()
        ).await
    }
}
