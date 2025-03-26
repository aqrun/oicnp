use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_PARENT_ID: &'static str = "idx-departments-parentId";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Departments::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Departments::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(Departments::Pid)
                .big_integer()
                .not_null()
                .default(0)
            )
            .col(ColumnDef::new(Departments::Name).string_len(32).not_null().default(""))
            .col(ColumnDef::new(Departments::Weight).small_integer().not_null().default(0))
            .col(ColumnDef::new(Departments::Leader).string_len(20).not_null().default(""))
            .col(ColumnDef::new(Departments::Phone).string_len(11).not_null().default(""))
            .col(ColumnDef::new(Departments::Email).string_len(50).not_null().default(""))
            .col(ColumnDef::new(Departments::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Departments::CreatedBy).big_integer().not_null().default(0))
            .col(ColumnDef::new(Departments::UpdatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(Departments::CreatedAt)
                    .date_time()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Departments::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Departments::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        // println!("---{:?}", table.to_string(PostgresQueryBuilder));

        let idx_parent_id = Index::create()
            .if_not_exists()
            .name(INDEX_PARENT_ID)
            .table(Departments::Table)
            .col(Departments::Pid)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_parent_id).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_PARENT_ID)
                .table(Departments::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(Departments::Table).to_owned()
        ).await
    }
}
