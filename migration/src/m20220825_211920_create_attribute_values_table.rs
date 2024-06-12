use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &'static str = "idx-attribute-values-vid";

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(AttributeValues::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AttributeValues::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(AttributeValues::Vid).string_len(100).not_null())
            .col(ColumnDef::new(AttributeValues::Label).string_len(100).not_null())
            .col(ColumnDef::new(AttributeValues::Value).string_len(100).not_null())
            .col(ColumnDef::new(AttributeValues::Weight).small_integer().not_null().default(0))
            .col(ColumnDef::new(AttributeValues::CssClass).string_len(100).not_null().default(""))
            .col(ColumnDef::new(AttributeValues::ListClass).string_len(100).not_null().default(""))
            .col(ColumnDef::new(AttributeValues::IsDefault).char_len(1).not_null().default(""))
            .col(ColumnDef::new(AttributeValues::Status).char_len(1).not_null().default(1))
            .col(ColumnDef::new(AttributeValues::Remark).string_len(500).not_null().default(""))
            .col(ColumnDef::new(AttributeValues::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(AttributeValues::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(AttributeValues::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(AttributeValues::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(AttributeValues::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(AttributeValues::Table)
            .col(AttributeValues::Vid)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_vid).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(AttributeValues::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(AttributeValues::Table).to_owned()
        ).await
    }
}
