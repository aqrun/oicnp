use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &'static str = "idx-crons-vid";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211935_create_crons_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysCrons::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysCrons::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysCrons::Vid).string_len(100).not_null())
            .col(ColumnDef::new(SysCrons::Count).integer().default(0))
            .col(ColumnDef::new(SysCrons::RunCount).interger().default(0))
            .col(ColumnDef::new(SysCrons::Name).string_len(64).not_null())
            .col(ColumnDef::new(SysCrons::Params).string_len(200).default(""))
            .col(ColumnDef::new(SysCrons::Group).string_len(64).default("DEFAULT"))
            .col(ColumnDef::new(SysCrons::InvokeTarget).string_len(500).default(""))
            .col(ColumnDef::new(SysCrons::Expression).string_len(255).default(""))
            .col(ColumnDef::new(SysCrons::MisfirePolicy).string_len(20).default(""))
            .col(ColumnDef::new(SysCrons::Concurrent).char_len(1).default("1"))
            .col(ColumnDef::new(SysCrons::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysCrons::Remark).string_len(500).default(""))
            .col(
                ColumnDef::new(SysCrons::LastTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysCrons::NextTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
            ColumnDef::new(SysCrons::EndTime)
                .date_time()
                .default(Value::Int(None)),
             )
            .col(ColumnDef::new(SysCrons::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysCrons::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysCrons::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysCrons::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysCrons::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(SysCrons::Table)
            .col(SysCrons::AttributeVid)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_parent_id).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(SysCrons::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(SysCrons::Table).to_owned()
        ).await
    }
}
