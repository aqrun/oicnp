use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(RoleDepartmentMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RoleDepartmentMap::RoleId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RoleDepartmentMap::DptId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RoleDepartmentMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(RoleDepartmentMap::RoleId)
                    .col(RoleDepartmentMap::DptId)
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(RoleDepartmentMap::Table).to_owned()
        ).await
    }
}
