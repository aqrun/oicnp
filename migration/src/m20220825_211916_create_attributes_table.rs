use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &'static str = "idx-attributes-vid";

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Attributes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Attributes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Attributes::Vid).string_len(100).not_null())
            .col(ColumnDef::new(Attributes::Name).string_len(100).not_null())
            .col(ColumnDef::new(Attributes::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Attributes::Remark).string_len(500).not_null().default(""))
            .col(ColumnDef::new(Attributes::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Attributes::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Attributes::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Attributes::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Attributes::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(Attributes::Table)
            .col(Attributes::Vid)
            .unique()
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_vid).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(Attributes::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(Attributes::Table).to_owned()
        ).await
    }
}
