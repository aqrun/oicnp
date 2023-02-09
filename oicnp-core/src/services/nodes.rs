use anyhow::{anyhow, Result};
use crate::models::{Node, NodeBody, NewNode, Taxonomies, DetailNode};
use crate::typings::{
    BodyFormat, NodeBundle, Count,
};
use crate::{DatabaseConnection};
use crate::entities::{
    cms_nodes, cms_node_body, cms_node_taxonomies_map, cms_taxonomies,
    prelude::{
        CmsNodes, CmsNodeBody, CmsNodeTaxonomiesMap, CmsTaxonomies,
    },
};
use sea_orm::*;
use sea_query::{Alias, Expr};
use log::{info};
use crate::utils::uuid;

pub async fn find_detail_nodes(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str, // DESC
    offset: &i32,
    limit: &i32,
) -> Result<Vec<DetailNode>> {
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
    order_dir: &str, // DESC
    offset: i32,
    limit: i32,
) -> Result<Vec<Node>> {
    let mut query = CmsNodes::find()
        .select_only()
        .column(cms_nodes::Column::Nid)
        .column(cms_nodes::Column::Vid)
        .column(cms_nodes::Column::Bundle)
        .column(cms_nodes::Column::Title)
        .column_as(
            Expr::tbl(Alias::new("nb"), cms_node_body::Column::Body).into_simple_expr(),
            "body"
        )
        .join_as(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(CmsNodeBody)
                .from(cms_nodes::Column::Nid)
                .to(cms_node_body::Column::Nid)
                .into(),
            Alias::new("nb")
        ).filter(
            Condition::all()
                .add(cms_nodes::Column::Deleted.eq("0"))
                .add(cms_nodes::Column::Bundle.eq(bundle))
        );

    query = query.order_by_desc(cms_nodes::Column::CreatedAt);

    // let data = query.into_model::<Node>()
    //     // .build(DbBackend::Postgres)
    //     // .to_string()
    //     .all(db)
    //     .await
    //     ;

    // 获取全部数据条数据
    let total = query.clone().count(db).await?;
    let pager = query.paginate(db, limit as usize);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(offset as usize).await?;

    println!("----1111111111--------{:?} total:{:?}  taotal_page: {:?}", list, total, total_pages);
    info!("{:?}", list);
    Err(anyhow!(""))
}

pub async fn find_nodes_count(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
) -> Result<Count> {
    todo!()
}


pub async fn find_node_by_vid(db: &DatabaseConnection, vid: &str, bundle: &NodeBundle) -> Result<Node> {
    // let w = rb.new_wrapper()
    //     .eq("vid", vid)
    //     .eq("bundle", bundle.to_string());
    // let node: Result<Option<Nodes>, Error> = rb.fetch_by_wrapper(w).await;

    // if let Ok(node) = node {
    //     if let Some(node) = node {
    //         return Ok(node);
    //     }
    // }
    Err(anyhow!("Node not exist: {}", ""))
}

pub async fn find_node_by_nid(db: &DatabaseConnection, nid: i32, bundle: &NodeBundle) -> Result<Node> {
    // let w = rb.new_wrapper()
    //     .eq("nid", nid)
    //     .eq("bundle", bundle.to_string());
    // let node: Result<Option<Nodes>, Error> = rb.fetch_by_wrapper(w).await;

    // if let Ok(node) = node {
    //     if let Some(node) = node {
    //         return Ok(node);
    //     }
    // }
    Err(anyhow!("Node not exist: {}", 1))
}

pub async fn find_node_body(db: &DatabaseConnection, nid: i32) -> Result<NodeBody> {
    // let node_body: Result<Option<NodeBody>, Error> = rb.fetch_by_column("nid", nid).await;
    // if let Ok(node_body) = node_body {
    //     if let Some(node_body) = node_body {
    //         return Ok(node_body);
    //     }
    // }
    Err(anyhow!("NodeBody not exist: {}", 1))
}

pub async fn save_node_content(
    db: &DatabaseConnection,
    nid: i32,
    body: &str,
    body_format: BodyFormat,
    summary: &str,
) -> Result<String> {
    // if let Err(err) = rb.remove_by_column::<NodeBody, _>("nid", nid).await {
    //     return Err(format!("Body save failed, {}", err.to_string()));
    // }

    // let node_body = NodeBody {
    //     nid,
    //     summary: String::from(summary),
    //     body: String::from(body),
    //     body_format: body_format.to_string(),
    // };
    // let res = rb.save(&node_body, &[]).await;

    // match res {
    //     Ok(_) => Ok(format!("Body save success")),
    //     Err(err) => Err(format!("Body save failed, {}", err.to_string())),
    // }
    Ok(format!(""))
}

pub async fn save_node(
    db: &DatabaseConnection,
    new_node: &NewNode,
    bundle: &NodeBundle,
) -> Result<Node> {
    if let Ok(node) = find_node_by_vid(db, &new_node.vid, bundle).await {
        return Ok(node);
    }

    let node = cms_nodes::ActiveModel {
        nid: Set(uuid()),
        vid: Set(Some(String::from(&new_node.vid))),
        bundle: Set(Some(String::from(&new_node.bundle))),
        title: Set(Some(String::from(&new_node.title))),
        viewed: Set(Some(0)),
        deleted: Set(Some("0".to_owned())),
        ..Default::default()
    };

    let node: cms_nodes::Model = node.insert(db).await?;

    // Ok(node)
    Err(anyhow!("Node save failed: {}", 1))
}

pub async fn find_node_taxonomies(
    db: &DatabaseConnection,
    bundle: &str,
    nid: &i32,
) -> Result<Vec<Taxonomies>> {
 todo!()
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
    order_dir: &str, // DESC
    limit: &i32,
    target_nid: &i32
) -> Result<Vec<DetailNode>> {
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
    let data: Vec<DetailNode> = Vec::new();

    Ok(data)
}