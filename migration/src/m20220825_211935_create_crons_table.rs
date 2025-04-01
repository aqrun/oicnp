use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_VID: &str = "idx-crons-vid";

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Crons::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Crons::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Crons::Vid).string_len(100).not_null())
            .col(ColumnDef::new(Crons::Count).integer().not_null().default(0))
            .col(ColumnDef::new(Crons::RunCount).integer().not_null().default(0))
            .col(ColumnDef::new(Crons::Name).string_len(64).not_null().default(""))
            .col(ColumnDef::new(Crons::Params).string_len(200).not_null().default(""))
            .col(ColumnDef::new(Crons::Group).string_len(64).not_null().default("DEFAULT"))
            .col(ColumnDef::new(Crons::InvokeTarget).string_len(500).not_null().default(""))
            .col(ColumnDef::new(Crons::Expression).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Crons::MisfirePolicy).string_len(20).not_null().default(""))
            .col(ColumnDef::new(Crons::Concurrent).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Crons::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Crons::Remark).string_len(500).not_null().default(""))
            .col(
                ColumnDef::new(Crons::LastTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Crons::NextTime)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
            ColumnDef::new(Crons::EndTime)
                .date_time()
                .default(Value::Int(None)),
             )
            .col(ColumnDef::new(Crons::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Crons::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Crons::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Crons::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Crons::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        let idx_vid = Index::create()
            .if_not_exists()
            .name(INDEX_VID)
            .table(Crons::Table)
            .col(Crons::Vid)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_vid).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_VID)
                .table(Crons::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(Crons::Table).to_owned()
        ).await
    }
}
