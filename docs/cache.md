```rust
// sys_caches
#[derive(Iden, OicColumn)]
pub enum Caches {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "CacheID")]
    Id,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    CacheKey,
    #[oic(data_type = "string", len = 512, default = "", comment = "")]
    CacheValue,
    #[oic(data_type = "string", len = 32, default = "", comment = "缓存类型")]
    Scope,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", comment = "创建时间")]
    ExpiredAt,
}
```

```rust
use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::sea_query::Expr;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Caches::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Caches::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Caches::CacheKey).string_len(64).not_null())
                    .col(ColumnDef::new(Caches::CacheValue).string_len(512).not_null())
                    .col(ColumnDef::new(Caches::Scope).string_len(32).not_null())
                    .col(
                        date_time(Caches::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time_null(Caches::ExpiredAt)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Caches::Table).to_owned())
            .await
    }
}
```