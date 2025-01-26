use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Uid)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Users::Uuid).string_len(64).unique_key().not_null())
            .col(ColumnDef::new(Users::Username).string_len(64).unique_key().not_null().default(""))
            .col(ColumnDef::new(Users::Nickname).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Users::Password).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Users::Salt).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Users::ApiKey).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Users::ResetToken).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Users::ResetSentAt).date_time().default(Value::Int(None)))
            .col(ColumnDef::new(Users::EmailVerifyToken).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Users::EmailVerifySentAt).date_time().default(Value::Int(None)))
            .col(ColumnDef::new(Users::EmailVerifiedAt).date_time().default(Value::Int(None)))
            .col(ColumnDef::new(Users::Status).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Users::Email).string_len(100).not_null().default(""))
            .col(ColumnDef::new(Users::Gender).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Users::Avatar).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Users::RoleId).big_integer().not_null().default(0))
            .col(ColumnDef::new(Users::DptId).big_integer().not_null().default(0))
            .col(ColumnDef::new(Users::Remark).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Users::IsAdmin).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Users::Phone).string_len(20).not_null().default(""))
            .col(ColumnDef::new(Users::LastLoginIp).string_len(20).not_null().default(""))
            .col(ColumnDef::new(Users::LastLoginAt).date_time().default(Value::Int(None)))
            .col(ColumnDef::new(Users::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Users::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Users::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Users::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Users::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Users::Table).to_owned()
        ).await
    }
}
