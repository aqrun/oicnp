---
title: Rust SeaORM 多表关联查询
description: 'Rust SeaORM LeftJoin 多表关联查询生成'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'seaorm', 'posgresql']
---

## 用到的表：

- nodes 节点内容表
- node_body 主内容
- taxonomies 分类表
- node_taxonomies_map 节点分类关联表
- users 用户表

## 最终数据 model

实现 FromQueryResult trait，可以将查询结果转为当前类型

```rust
#[derive(Debug, Clone, FromQueryResult)]
pub struct DetailNode {
    pub nid: String,
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    pub created_at: DateTime,
    pub created_by: String,
    pub updated_at: DateTime,
    pub updated_by: String,
    pub updated_by_username: Option<String>,
    pub updated_by_nickname: Option<String>,

    pub tid: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: Option<String>,
    pub author_username: Option<String>,
    pub author_nickname: Option<String>,

    pub summary: String,
    pub body: String,
    pub body_format: String,
}
```

## 查询操作

```rust
pub async fn find_nodes(
    db: &DatabaseConnection,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str, // DESC
    offset: i32,
    limit: i32,
) -> Result<Vec<Node>> {
    let mut query = CmsNodes::find()
        .select_only()
        // 指定主表字段
        .columns([
            cms_nodes::Column::Nid,
            cms_nodes::Column::Vid,
            cms_nodes::Column::Bundle,
            cms_nodes::Column::Title,
            cms_nodes::Column::Viewed,
            cms_nodes::Column::Deleted,
            cms_nodes::Column::Deleted,
            cms_nodes::Column::PublishedAt,
            cms_nodes::Column::CreatedBy,
            cms_nodes::Column::UpdatedBy,
            cms_nodes::Column::CreatedAt,
            cms_nodes::Column::UpdatedAt,
            cms_nodes::Column::DeletedAt,
        ])
        // 指定关联表字段
        .column_as(cms_node_body::Column::Summary, "summary")
        .column_as(cms_node_body::Column::Body, "body")
        .column_as(cms_node_body::Column::BodyFormat, "body_format")
        .column_as(cms_taxonomies::Column::Tid, "tid")
        .column_as(cms_taxonomies::Column::Vid, "category_vid")
        .column_as(cms_taxonomies::Column::Name, "category_name")
        .column_as(
            Expr::col((Alias::new("cu"), sys_users::Column::Uid)),
            "author_uid"
        )
        .column_as(
            Expr::col((Alias::new("cu"), sys_users::Column::Username)),
            "author_username"
        )
        .column_as(
            Expr::col((Alias::new("cu"), sys_users::Column::Nickname)),
            "author_nickname"
        )
        .column_as(
            Expr::col((Alias::new("uu"), sys_users::Column::Username)),
            "updated_by_username"
        )
        .column_as(
            Expr::col((Alias::new("uu"), sys_users::Column::Nickname)),
            "updated_by_nickname"
        )
        .join(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(CmsNodeBody)
                .from(cms_nodes::Column::Nid)
                .to(cms_node_body::Column::Nid)
                .into()
        )
        .join(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(CmsNodeTaxonomiesMap)
                .from(cms_nodes::Column::Nid)
                .to(cms_node_taxonomies_map::Column::Nid)
                .into()
        )
        .join(
            JoinType::LeftJoin,
            CmsNodeTaxonomiesMap::belongs_to(CmsTaxonomies)
                .from(cms_node_taxonomies_map::Column::Tid)
                .to(cms_taxonomies::Column::Tid)
                .into(),
        )
        // 关联用户表 指定别名 cu 创建者信息
        .join_as(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(SysUsers)
                .from(cms_nodes::Column::CreatedBy)
                .to(sys_users::Column::Uid)
                .into(),
            Alias::new("cu"),
        )
        // 关联用户表 指定别名 uu 更新者信息
        .join_as(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(SysUsers)
                .from(cms_nodes::Column::UpdatedBy)
                .to(sys_users::Column::Uid)
                .into(),
            Alias::new("uu"),
        )
        .filter(
            Condition::all()
                .add(cms_nodes::Column::Deleted.eq("0"))
                .add(cms_nodes::Column::Bundle.eq("article"))
        );

    query = query.order_by_desc(cms_nodes::Column::CreatedAt);

    // 生成查询sql
    let sql = query.clone()
        .build(DbBackend::Postgres)
        .to_string();

    println!("sql-----: {:?}", sql);
    // 获取全部数据条数据
    let total = query.clone().count(db).await?;
    // 分页查询并将结果转为 DetailNode 数据类型
    let pager = query
        .into_model::<DetailNode>()
        .paginate(db, limit as usize);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(offset as usize).await?;

    println!("list-{:?} total:{:?}  taotal_page: {:?}", list, total, total_pages);

    Err(anyhow!(""))
}
```

## 生成的 sql

```sql
SELECT
"cms_nodes"."nid", "cms_nodes"."vid",
"cms_nodes"."bundle", "cms_nodes"."title",
"cms_nodes"."viewed", "cms_nodes"."deleted",
"cms_nodes"."deleted", "cms_nodes"."published_at",
"cms_nodes"."created_by", "cms_nodes"."updated_by",
"cms_nodes"."created_at", "cms_nodes"."updated_at",
"cms_nodes"."deleted_at",
"cms_node_body"."summary" AS "summary",
"cms_node_body"."body" AS "body",
"cms_node_body"."body_format" AS "body_format",
"cms_taxonomies"."tid" AS "tid",
"cms_taxonomies"."vid" AS "category_vid",
"cms_taxonomies"."name" AS "category_name",
"cu"."uid" AS "author_uid",
"cu"."username" AS "author_username",
"cu"."nickname" AS "author_nickname",
"uu"."username" AS "updated_by_username",
"uu"."nickname" AS "updated_by_nickname"
FROM "cms_nodes"
LEFT JOIN "cms_node_body"
      ON "cms_nodes"."nid" = "cms_node_body"."nid"
LEFT JOIN "cms_node_taxonomies_map"
      ON "cms_nodes"."nid" = "cms_node_taxonomies_map"."nid"
LEFT JOIN "cms_taxonomies"
      ON "cms_node_taxonomies_map"."tid" = "cms_taxonomies"."tid"
LEFT JOIN "sys_users" AS "cu"
      ON "cms_nodes"."created_by" = "cu"."uid"
LEFT JOIN "sys_users" AS "uu"
      ON "cms_nodes"."updated_by" = "uu"."uid"
WHERE
      "cms_nodes"."deleted" = '0'
      AND "cms_nodes"."bundle" = 'article'
ORDER BY "cms_nodes"."created_at" DESC
```
