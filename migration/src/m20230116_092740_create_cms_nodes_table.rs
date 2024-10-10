use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Nodes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Nodes::Nid)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Nodes::Uuid).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Nodes::Vid).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Nodes::Bundle).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Nodes::Title).string_len(512).not_null().default(""))
            .col(ColumnDef::new(Nodes::Viewed).integer().not_null().default(0))
            .col(ColumnDef::new(Nodes::Deleted).char_len(1).not_null().default("0"))
            .col(
                ColumnDef::new(Nodes::PublishedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(ColumnDef::new(Nodes::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Nodes::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Nodes::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Nodes::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Nodes::DeletedAt)
                    .date_time()
                    .default(Value::Int(None))
            )
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Nodes::Table).to_owned())
            .await
    }
}
