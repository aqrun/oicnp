use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UpdateLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UpdateLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(UpdateLogs::AppVersion).string_len(10).not_null().default(""))
            .col(ColumnDef::new(UpdateLogs::BackendVersion).string_len(10).not_null().default(""))
            .col(ColumnDef::new(UpdateLogs::Title).string_len(100).not_null().default(""))
            .col(ColumnDef::new(UpdateLogs::Content).text().not_null().default(""))
            .col(ColumnDef::new(UpdateLogs::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(UpdateLogs::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(UpdateLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(UpdateLogs::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(UpdateLogs::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(UpdateLogs::Table).to_owned()
        ).await
    }
}
