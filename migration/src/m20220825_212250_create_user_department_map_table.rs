use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UserDepartmentMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserDepartmentMap::Uid)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(UserDepartmentMap::DptId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(UserDepartmentMap::CreatedBy).big_integer().default(0))
            .col(
                ColumnDef::new(UserDepartmentMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(UserDepartmentMap::Uid)
                    .col(UserDepartmentMap::DptId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(UserDepartmentMap::Table).to_owned()
        ).await
    }
}
