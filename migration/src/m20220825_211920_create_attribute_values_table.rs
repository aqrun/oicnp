use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &'static str = "idx-attribute-values-vid";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211920_create_attribute_values_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysAttributeValues::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysAttributeValues::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysAttributeValues::AttributeVid).string_len(100).not_null())
            .col(ColumnDef::new(SysAttributeValues::Label).string_len(100).not_null())
            .col(ColumnDef::new(SysAttributeValues::Value).string_len(100).not_null())
            .col(ColumnDef::new(SysAttributeValues::Weight).small_integer().default(0))
            .col(ColumnDef::new(SysAttributeValues::CssClass).string_len(100).default(""))
            .col(ColumnDef::new(SysAttributeValues::ListClass).string_len(100).default(""))
            .col(ColumnDef::new(SysAttributeValues::IsDefault).char_len(1).default(""))
            .col(ColumnDef::new(SysAttributeValues::Status).char_len(1).default(1))
            .col(ColumnDef::new(SysAttributeValues::Remark).string_len(500).default(""))
            .col(ColumnDef::new(SysAttributeValues::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysAttributeValues::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysAttributeValues::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysAttributeValues::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysAttributeValues::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(SysAttributeValues::Table)
            .col(SysAttributeValues::AttributeVid)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_vid).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(SysAttributeValues::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(SysAttributeValues::Table).to_owned()
        ).await
    }
}
