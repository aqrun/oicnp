// use oicnp_core::prelude::sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// impl MigrationName for Migration {
//     fn name(&self) -> &str {
//         "m20230116_090508_create_cms_files_table"
//     }
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let cms_files_table = Table::create()
            .table(Files::table_name(""))
            .if_not_exists()
            .col(
                ColumnDef::new(Files::Fid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(Files::Uid).string_len(32).default(""))
            .col(ColumnDef::new(Files::Filename).string_len(255).default(""))
            .col(ColumnDef::new(Files::Uri).string_len(255).default(""))
            .col(ColumnDef::new(Files::Storage).string_len(64).default(""))
            .col(ColumnDef::new(Files::Mime).string_len(64).default(""))
            .col(ColumnDef::new(Files::Status).char_len(1).default("0"))
            .col(ColumnDef::new(Files::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(Files::UpdatedBy).string_len(32).default(""))
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

