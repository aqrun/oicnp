use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212238_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUsers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUsers::Uid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysUsers::Username).string_len(60).default(""))
            .col(ColumnDef::new(SysUsers::Nickname).string_len(60).default(""))
            .col(ColumnDef::new(SysUsers::Password).string_len(64).default(""))
            .col(ColumnDef::new(SysUsers::Salt).string_len(64).default(""))
            .col(ColumnDef::new(SysUsers::Status).char_len(1).default("0"))
            .col(ColumnDef::new(SysUsers::Email).string_len(100).default(""))
            .col(ColumnDef::new(SysUsers::Gender).char_len(1).default("0"))
            .col(ColumnDef::new(SysUsers::Avatar).string_len(32).default(""))
            .col(ColumnDef::new(SysUsers::RoleId).string_len(32).default(""))
            .col(ColumnDef::new(SysUsers::DepartmentId).string_len(32).default(""))
            .col(ColumnDef::new(SysUsers::Remark).string_len(255).default(""))
            .col(ColumnDef::new(SysUsers::IsAdmin).char_len(1).default("0"))
            .col(ColumnDef::new(SysUsers::Phone).string_len(20).default(""))
            .col(ColumnDef::new(SysUsers::LastLoginIp).string_len(20).default(""))
            .col(ColumnDef::new(SysUsers::LastLoginAt).date_time().default(Value::Int(None)))
            .col(ColumnDef::new(SysUsers::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysUsers::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysUsers::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysUsers::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysUsers::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUsers::Table).to_owned()
        ).await
    }
}
