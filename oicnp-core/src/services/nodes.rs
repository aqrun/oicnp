use crate::entities::{
    node_body, node, category,
    node_tags_map, tag,
    prelude::*,
    user,
};
use crate::models::{
    NewNode, Node
};
use crate::typings::{BodyFormat, ListData, NodeBundle};
use crate::utils::uuid;
use crate::DatabaseConnection;
use crate::services::update_tag_count_by_id;
use anyhow::{anyhow, Result};
use sea_orm::*;
use sea_query::{Alias, Expr};

pub async fn find_detail_nodes(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str,  // DESC
    offset: &i32,
    limit: &i32,
) -> Result<Vec<Node>> {
    todo!()
}

/**
* SELECT n.*
 FROM nodes n
 LEFT JOIN node_body nb ON n.nid=nb.nid
 LEFT JOIN node_taxonomies_map ntm ON ntm.nid=n.nid
 LEFT JOIN taxonomies t ON t.tid=ntm.tid
 LEFT JOIN users cu ON n.created_by=cu.uid
 LEFT JOIN users uu on n.updated_by=uu.uid
 LEFT JOIN users a ON n.uid=a.uid
 WHERE n.deleted = false
 AND n.bundle = '${bundle}'
 AND t.bundle = 'category'

 for index,item in filters:
   AND ${item}

 if category != '':
   AND t.name = '${category}'

 ORDER BY n.${order_name} ${order_dir}
 OFFSET ${offset}
 LIMIT ${limit}
*/
pub async fn find_nodes(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str,  // DESC
    page: u64,
    page_size: u64,
) -> Result<ListData<Node>> {
    let mut query = NodeEntity::find()
        .select_only()
        .columns([
            node::Column::Nid,
            node::Column::Vid,
            node::Column::Bundle,
            node::Column::Title,
            node::Column::Viewed,
            node::Column::Deleted,
            node::Column::Deleted,
            node::Column::PublishedAt,
            node::Column::CreatedBy,
            node::Column::UpdatedBy,
            node::Column::CreatedAt,
            node::Column::UpdatedAt,
            node::Column::DeletedAt,
        ])
        .column_as(node_body::Column::Summary, "summary")
        .column_as(node_body::Column::SummaryFormat, "summary_format")
        .column_as(node_body::Column::Body, "body")
        .column_as(node_body::Column::BodyFormat, "body_format")
        .column_as(category::Column::CatId, "tid")
        .column_as(category::Column::CatVid, "category_vid")
        .column_as(category::Column::CatName, "category_name")
        .column_as(
            Expr::col((Alias::new("cu"), user::Column::Uid)),
            "author_uid",
        )
        .column_as(
            Expr::col((Alias::new("cu"), user::Column::Username)),
            "author_username",
        )
        .column_as(
            Expr::col((Alias::new("cu"), user::Column::Nickname)),
            "author_nickname",
        )
        .column_as(
            Expr::col((Alias::new("uu"), user::Column::Username)),
            "updated_by_username",
        )
        .column_as(
            Expr::col((Alias::new("uu"), user::Column::Nickname)),
            "updated_by_nickname",
        )
        .join(
            JoinType::LeftJoin,
            NodeEntity::belongs_to(NodeBodyEntity)
                .from(node::Column::Nid)
                .to(node_body::Column::Nid)
                .into(),
        )
        .join(
            JoinType::LeftJoin,
            NodeEntity::belongs_to(NodeCategoriesMapEntity)
                .from(node::Column::Nid)
                .to(NodeCategoriesMapColumn::Nid)
                .into(),
        )
        .join(
            JoinType::LeftJoin,
            NodeCategoriesMapEntity::belongs_to(CategoryEntity)
                .from(NodeCategoriesMapColumn::CatId)
                .to(category::Column::CatId)
                .into(),
        )
        .join_as(
            JoinType::LeftJoin,
            NodeEntity::belongs_to(UserEntity)
                .from(node::Column::CreatedBy)
                .to(user::Column::Uid)
                .into(),
            Alias::new("cu"),
        )
        .join_as(
            JoinType::LeftJoin,
            NodeEntity::belongs_to(UserEntity)
                .from(node::Column::UpdatedBy)
                .to(user::Column::Uid)
                .into(),
            Alias::new("uu"),
        )
        .filter(node::Column::Deleted.eq("0"));

    if !bundle.is_empty() {
        query = query.filter(node::Column::Bundle.eq(bundle));
    }

    if !category.is_empty() {
        query = query.filter(category::Column::CatVid.eq(category));
    }

    query = query.order_by_desc(node::Column::CreatedAt);

    /*
    let data = query.clone()
        // .into_model::<Node>()
        .build(DbBackend::Postgres)
        .to_string()
        // .all(db)
        // .await
        ;
     */

    // 获取全部数据条数据
    let total = query.clone().count(db).await?;
    let pager = query
        .into_model::<Node>()
        .paginate(db, page_size);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(page - 1).await?;

    let list_data: ListData<Node> = ListData {
        data: list,
        page,
        page_size,
        total_pages,
        total_count: total,
    };

    Ok(list_data)
}

pub async fn find_nodes_count(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
) -> Result<i32> {
    let q: Select<NodeEntity> = NodeEntity::find()
        .select_only()
        .column(NodeColumn::Nid)
        .join(
            JoinType::LeftJoin,
            NodeEntity::belongs_to(NodeCategoriesMapEntity)
                .from(NodeColumn::Nid)
                .to(NodeCategoriesMapColumn::Nid)
                .into(),
        )
        .join(
            JoinType::LeftJoin,
            NodeCategoriesMapEntity::belongs_to(CategoryEntity)
                .from(NodeCategoriesMapColumn::CatId)
                .to(CategoryColumn::CatId)
                .into(),
        )
        .filter(
            Condition::all()
                .add(NodeColumn::Deleted.eq("0"))
                .add(NodeColumn::Bundle.eq(bundle))
                .add(CategoryColumn::CatName.eq(category)),
        );

    /*
    let data = // q.into_model::<Node>()
        q.build(DbBackend::Postgres)
        .to_string()
        // .all(db)
        // .await
        ;
    println!("data: {:?}", data);
    */
    let total = q.count(db).await?;

    Ok(total as i32)
}

pub async fn find_node_by_vid(
    db: &DatabaseConnection,
    vid: &str,
    bundle: &NodeBundle,
) -> Result<Node> {
    let mut q = NodeEntity::find();
    q = q.filter(node::Column::Vid.eq(vid));
    q = q.filter(node::Column::Bundle.eq(bundle.to_string()));
    q = q.filter(node::Column::Deleted.eq("0"));
    // let a = q.clone().into_json().one(db).await?;
    let res: Option<Node> = q.clone().into_model::<Node>().one(db).await?;
    // let b = q.clone().into_model::<node::Model>().one(db).await?;

    if let Some(node) = res {
        return Ok(node);
    }

    Err(anyhow!("Node not exist: {}", vid))
}

pub async fn find_node_by_nid(
    db: &DatabaseConnection,
    nid: i64,
    bundle: &NodeBundle,
) -> Result<Node> {
    let mut q = NodeEntity::find();
    q = q.filter(node::Column::Nid.eq(nid));
    q = q.filter(node::Column::Bundle.eq(bundle.to_string()));
    q = q.filter(node::Column::Deleted.eq("0"));

    let res = q.into_model::<Node>().one(db).await?;

    if let Some(node) = res {
        return Ok(node);
    }
    Err(anyhow!("Node not exist: {}", 1))
}

pub async fn find_node_body(db: &DatabaseConnection, nid: i64) -> Result<NodeBodyModel> {
    let mut q = NodeBodyEntity::find();
    q = q.filter(node_body::Column::Nid.eq(nid));

    let res = q.into_model::<NodeBodyModel>().one(db).await?;

    if let Some(node_body) = res {
        return Ok(node_body);
    }

    Err(anyhow!("NodeBody not exist: {}", 1))
}

pub async fn save_node_content(
    db: &DatabaseConnection,
    nid: i64,
    body: &str,
    body_format: BodyFormat,
    summary: &str,
) -> Result<i64> {
    let node_body = node_body::ActiveModel {
        nid: Set(nid),
        summary: Set(String::from(summary)),
        summary_format: Set(body_format.to_string()),
        body: Set(String::from(body)),
        body_format: Set(body_format.to_string()),
        ..Default::default()
    };

    let res: node_body::Model = match node_body.insert(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Node Body save failed {}", err.to_string()));
        }
    };

    Ok(res.nid)
}

pub async fn save_node(
    db: &DatabaseConnection,
    new_node: &NewNode,
    bundle: &NodeBundle,
) -> Result<Node> {
    if let Ok(node) = find_node_by_vid(db, &new_node.vid, bundle).await {
        return Ok(node);
    }

    let node = node::ActiveModel {
        // nid: Set(uuid()),
        uuid: Set(uuid()),
        vid: Set(String::from(&new_node.vid)),
        bundle: Set(String::from(&new_node.bundle)),
        title: Set(String::from(&new_node.title)),
        viewed: Set(0),
        deleted: Set("0".to_owned()),
        published_at: Set(new_node.published_at),
        created_by: Set(new_node.created_by),
        updated_by: Set(new_node.updated_by),
        created_at: Set(new_node.created_at),
        updated_at: Set(Some(new_node.updated_at)),
        deleted_at: Set(None),
        ..Default::default()
    };

    let node = node.insert(db).await?;

    let data = find_node_by_nid(
        db,
        node.nid, 
        &NodeBundle::Article
    ).await?;
    Ok(data)
}

pub async fn save_node_categories_map(
    db: &DatabaseConnection,
    bundle: &str,
    nid: i64,
    tid: i64,
) -> Result<NodeCategoriesMapModel> {
    let n = NodeCategoriesMapActiveModel {
        bundle: Set(String::from(bundle)),
        nid: Set(nid),
        cat_id: Set(tid),
    };
    let res = n.insert(db).await?;

    Ok(res)
}

pub async fn save_node_tags_map(
    db: &DatabaseConnection,
    bundle: &str,
    nid: i64,
    tag_id: i64,
) -> Result<NodeTagsMapModel> {
    let n = node_tags_map::ActiveModel {
        bundle: Set(String::from(bundle)),
        tag_id: Set(tag_id),
        nid: Set(nid),
    };
    let res = n.insert(db).await?;

    update_tag_count_by_id(db, tag_id).await?;

    Ok(res)
}

/// 获取节点全部分类
pub async fn find_node_categories(db: &DatabaseConnection, nid: i64) -> Result<Vec<CategoryModel>> {
    let q = CategoryEntity::find()
        .join(
            JoinType::LeftJoin,
            CategoryEntity::belongs_to(NodeCategoriesMapEntity)
                .from(category::Column::CatId)
                .to(NodeCategoriesMapColumn::CatId)
                .into(),
        )
        .filter(
            Condition::all().add(NodeCategoriesMapColumn::Nid.eq(nid)), // .add(cms_node_taxonomies_map::Column::Bundle.eq(bundle))
        );

    // let data = // q.into_model::<Node>()
    //     a.build(DbBackend::Postgres)
    //         .to_string();
    let res = q.into_model::<CategoryModel>().all(db).await?;
    return Ok(res);
    // println!("----{:?}", res);
}

pub async fn find_node_tags(db: &DatabaseConnection, nid: i64) -> Result<Vec<TagModel>> {
    let q = TagEntity::find()
        .join(
            JoinType::LeftJoin,
            TagEntity::belongs_to(NodeTagsMapEntity)
                .from(tag::Column::TagId)
                .to(node_tags_map::Column::TagId)
                .into(),
        )
        .filter(
            node_tags_map::Column::Nid.eq(nid)
        );

    let res = q.into_model::<TagModel>().all(db).await?;
    return Ok(res);
}

///
/// 获取指定 target_id 对应的 相关数据列表
///
pub async fn find_nodes_width_target_id(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str,  // DESC
    limit: &i32,
    target_nid: &i32,
) -> Result<Vec<Node>> {
    // let mut data: Vec<DetailNode> = vec!();
    // let mut target_arr: Vec<DetailNode> = vec![];
    // let mut temp_filters = filters.clone();

    // let mut prev_filters = vec![
    //     format!("n.nid = {}", target_nid)
    // ];
    // prev_filters.append(&mut temp_filters);

    // if let Ok(res) = find_detail_nodes(
    //     rb.clone(),
    //     &bundle,
    //     &category,
    //     &prev_filters,
    //     &order_name,
    //     &order_dir,
    //     &0,
    //     &1
    // ).await {
    //     target_arr = res;
    // }

    // if limit <= &1 {
    //     return Ok(target_arr);
    // }

    // let limit = (*limit as f32 / 2.0).ceil() as i32;
    // let mut next_filters = vec![
    //     format!("n.nid < {}", target_nid)
    // ];
    // next_filters.append(&mut temp_filters);

    // if let Ok(res) = find_detail_nodes(
    //     rb.clone(),
    //     &bundle,
    //     &category,
    //     &next_filters,
    //     &order_name,
    //     &order_dir,
    //     &0,
    //     &limit
    // ).await {
    //     let mut temp = res;
    //     temp.reverse();
    //     data.append(&mut temp);
    // }

    // data.extend(target_arr);

    // let filters = vec![
    //     format!("n.nid > {}", target_nid)
    // ];
    // let order_dir = String::from("ASC");
    // if let Ok(res) = find_detail_nodes(
    //     rb.clone(),
    //     &bundle,
    //     &category,
    //     &filters,
    //     &order_name,
    //     &order_dir,
    //     &0,
    //     &limit
    // ).await {
    //     let mut temp = res;
    //     data.append(&mut temp);
    // }
    let data: Vec<Node> = Vec::new();

    Ok(data)
}
