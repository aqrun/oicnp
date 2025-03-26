use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Permissions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Permissions::PermissionId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Permissions::Vid).string_len(64).unique_key().not_null())
            .col(ColumnDef::new(Permissions::Name).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Permissions::Api).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Permissions::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(Permissions::Scope).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Permissions::Status).char_len(1).not_null().default(""))
            .col(ColumnDef::new(Permissions::Remark).string_len(255).not_null().default(""))
            .col(
                ColumnDef::new(Permissions::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Permissions::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Permissions::Table).to_owned()
        ).await
    }
}
