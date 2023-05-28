use std::sync::Arc;
use crate::models::{Nodes, NodeBody, NewNode, Taxonomies};
use crate::typings::{
    BodyFormat, NodeBundle, DetailNode, Count,
};
use oicnp_core::{DatabaseConnection};
use anyhow::{anyhow, Result};
use oicnp_core::{
    entities::{
        cms_nodes,
    },
};

/*#[py_sql("
SELECT n.*, nb.body, nb.body_format, nb.summary,
  t.name AS category_name, t.vid as category_vid,
  t.bundle as category_bundle, t.tid,
  a.uid as author_uid, a.username as author_username,
  a.nickname as author_nickname,
  cu.username as created_by_username,
  cu.nickname as created_by_nickname,
  uu.username as updated_by_username,
  uu.nickname as updated_by_nickname
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
")]*/
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
    Err(anyhow!("map not exist"))
}

/*#[py_sql("
SELECT n.*
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
")]*/
pub async fn find_nodes(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str, // DESC
    offset: &i32,
    limit: &i32,
) -> Result<Vec<Nodes>> {
    Err(anyhow!("map not exist"))
}

/*#[py_sql("
SELECT count(n.nid) AS count
  FROM nodes n
  LEFT JOIN node_body nb ON n.nid=nb.nid
  LEFT JOIN node_taxonomies_map ntm ON ntm.nid=n.nid
  LEFT JOIN taxonomies t ON t.tid=ntm.tid
  LEFT JOIN users cu ON n.created_by=cu.uid
  LEFT JOIN users uu on n.updated_by=uu.uid
  LEFT JOIN users a ON n.uid=a.uid
  WHERE n.deleted = false
  AND n.bundle = #{bundle}
  AND t.bundle = 'category'

  if category != '':
    AND t.name = #{category}
")]*/
pub async fn find_nodes_count(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
) -> Result<Count> {
    Err(anyhow!("map not exist"))
}


pub async fn find_node_by_vid(db: &DatabaseConnection, vid: &str, bundle: &NodeBundle) -> Result<Nodes> {
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

pub async fn find_node_by_nid(db: &DatabaseConnection, nid: i32, bundle: &NodeBundle) -> Result<Nodes> {
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
) -> Result<Nodes> {
    // if let Ok(node) = find_node_by_vid(rb.clone(), &new_node.vid, bundle).await {
    //     return Ok(node);
    // }

    // let res = rb.save(&new_node, &[]).await;

    // if let Err(err) = res {
    //     return Err(err.to_string());
    // }

    // if let Ok(node) = find_node_by_vid(rb.clone(), &new_node.vid, bundle).await {
    //     return Ok(node);
    // }

    Err(anyhow!("Node save failed: {}", 1))
}

/*#[py_sql("
SELECT t.* FROM taxonomies t
  LEFT JOIN node_taxonomies_map ntm ON t.tid = ntm.tid
  WHERE ntm.nid=#{nid}
  AND t.bundle=#{bundle}
  ORDER BY count desc, weight
")]*/
pub async fn find_node_taxonomies(
    db: &DatabaseConnection,
    bundle: &str,
    nid: &i32,
) -> Result<Vec<Taxonomies>> {
    Err(anyhow!("map not exist"))
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
    let data = Vec::new();

    Ok(data)
}