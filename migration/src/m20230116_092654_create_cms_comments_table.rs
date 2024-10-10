use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Comments::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Comments::CommentId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Comments::Uid).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Comments::Pid).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Comments::Status).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Comments::Bundle).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Comments::TargetId).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Comments::Subject).string_len(512).not_null().default(""))
            .col(ColumnDef::new(Comments::Name).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Comments::Email).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Comments::Homepage).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Comments::Hostname).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Comments::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Comments::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Comments::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Comments::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None))
            )
            .col(
                ColumnDef::new(Comments::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}
