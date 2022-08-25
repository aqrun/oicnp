use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &'static str = "idx-attributes-vid";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211916_create_attributes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysAttributes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysAttributes::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysAttributes::Vid).string_len(100).not_null())
            .col(ColumnDef::new(SysAttributes::Name).string_len(100).not_null())
            .col(ColumnDef::new(SysAttributes::Status).char().not_null())
            .col(ColumnDef::new(SysAttributes::Remark).string_len(500).not_null())
            .col(ColumnDef::new(SysAttributes::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysAttributes::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysAttributes::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysAttributes::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysAttributes::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(SysAttributes::Table)
            .col(SysAttributes::Vid)
            .unique()
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_vid).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(SysAttributes::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(SysAttributes::Table).to_owned()
        ).await
    }
}
