use sea_orm_migration::prelude::*;
use super::types::*;

const INDEX_DB: &str = "idx-apiDb-db";

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ApiDb::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ApiDb::ApiId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(ApiDb::Db).string_len(32).not_null())
            .to_owned();

        let idx_db = Index::create()
            .if_not_exists()
            .name(INDEX_DB)
            .table(ApiDb::Table)
            .col(ApiDb::Db)
            .to_owned();

        manager.create_table(table).await?;
        manager.create_index(idx_db).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name(INDEX_DB)
                .table(ApiDb::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(
            Table::drop().table(ApiDb::Table).to_owned()
        ).await
    }
}

