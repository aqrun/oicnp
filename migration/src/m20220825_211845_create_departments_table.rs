use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_PARENT_ID: &'static str = "idx-departments-parentId";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211845_create_departments_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysDepartments::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysDepartments::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysDepartments::ParentId).string_len(32).not_null())
            .col(ColumnDef::new(SysDepartments::Name).string_len(32).default(""))
            .col(ColumnDef::new(SysDepartments::Weight).small_integer().default(0))
            .col(ColumnDef::new(SysDepartments::Leader).string_len(20).default(""))
            .col(ColumnDef::new(SysDepartments::Phone).string_len(11).default(""))
            .col(ColumnDef::new(SysDepartments::Email).string_len(50).default(""))
            .col(ColumnDef::new(SysDepartments::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysDepartments::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysDepartments::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysDepartments::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysDepartments::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysDepartments::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        // println!("---{:?}", table.to_string(PostgresQueryBuilder));

        let idx_parent_id = Index::create()
            .if_not_exists()
            .name(INDEX_PARENT_ID)
            .table(SysDepartments::Table)
            .col(SysDepartments::ParentId)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_parent_id).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_PARENT_ID)
                .table(SysDepartments::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(SysDepartments::Table).to_owned()
        ).await
    }
}
