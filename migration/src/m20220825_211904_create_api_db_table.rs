use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_DB: &'static str = "idx-apiDb-db";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_211904_create_api_db_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysApiDb::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysApiDb::ApiId)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysApiDb::Db).string_len(32).not_null())
            .to_owned();

        let idx_db = Index::create()
            .if_not_exists()
            .name(INDEX_DB)
            .table(SysApiDb::Table)
            .col(SysApiDb::Db)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_db).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_DB)
                .table(SysApiDb::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(SysApiDb::Table).to_owned()
        ).await
    }
}

