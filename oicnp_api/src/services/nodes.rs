use std::sync::Arc;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use crate::models::{Nodes, NodeBody, NewNode, Taxonomies};
use crate::typings::{
    BodyFormat, NodeBundle, DetailNode, Count,
};

#[py_sql("
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
  AND n.bundle = #{bundle}
  AND t.bundle = 'category'

  if category != '':
    AND t.name = #{category}

  ORDER BY n.created_at
  OFFSET #{offset}
  LIMIT #{limit}
")]
pub async fn find_nodes(
    rb: Arc<Rbatis>,
    bundle: &str,
    category: &str,
    offset: &i32,
    limit: &i32,
) -> Result<Vec<DetailNode>, Error> {
    todo!()
}

#[py_sql("
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
")]
pub async fn find_nodes_count(
    rb: Arc<Rbatis>,
    bundle: &str,
    category: &str,
) -> Result<Count, Error> {
    todo!()
}


pub async fn find_node_by_vid(rb: Arc<Rbatis>, vid: &str, bundle: &NodeBundle) -> Result<Nodes, String> {
    let w = rb.new_wrapper()
        .eq("vid", vid)
        .eq("bundle", bundle.to_string());
    let node: Result<Option<Nodes>, Error> = rb.fetch_by_wrapper(w).await;

    if let Ok(node) = node {
        if let Some(node) = node {
            return Ok(node);
        }
    }
    Err(format!("Node not exist: {}", vid))
}

pub async fn find_node_by_nid(rb: Arc<Rbatis>, nid: i32, bundle: &NodeBundle) -> Result<Nodes, String> {
    let w = rb.new_wrapper()
        .eq("nid", nid)
        .eq("bundle", bundle.to_string());
    let node: Result<Option<Nodes>, Error> = rb.fetch_by_wrapper(w).await;

    if let Ok(node) = node {
        if let Some(node) = node {
            return Ok(node);
        }
    }
    Err(format!("Node not exist: {}", nid))
}

pub async fn find_node_body(rb: Arc<Rbatis>, nid: i32) -> Result<NodeBody, String> {
    let node_body: Result<Option<NodeBody>, Error> = rb.fetch_by_column("nid", nid).await;
    if let Ok(node_body) = node_body {
        if let Some(node_body) = node_body {
            return Ok(node_body);
        }
    }
    Err(format!("NodeBody not exist: {}", nid))
}

pub async fn save_node_content(
    rb: Arc<Rbatis>,
    nid: i32,
    body: &str,
    body_format: BodyFormat,
    summary: &str,
) -> Result<String, String> {
    if let Err(err) = rb.remove_by_column::<NodeBody, _>("nid", nid).await {
        return Err(format!("Body save failed, {}", err.to_string()));
    }

    let node_body = NodeBody {
        nid,
        summary: String::from(summary),
        body: String::from(body),
        body_format: body_format.to_string(),
    };
    let res = rb.save(&node_body, &[]).await;

    match res {
        Ok(_) => Ok(format!("Body save success")),
        Err(err) => Err(format!("Body save failed, {}", err.to_string())),
    }
}

pub async fn save_node(rb: Arc<Rbatis>, new_node: &NewNode, bundle: &NodeBundle) -> Result<Nodes, String> {
    if let Ok(node) = find_node_by_vid(rb.clone(), &new_node.vid, bundle).await {
        return Ok(node);
    }

    let res = rb.save(&new_node, &[]).await;

    if let Err(err) = res {
        return Err(err.to_string());
    }

    if let Ok(node) = find_node_by_vid(rb.clone(), &new_node.vid, bundle).await {
        return Ok(node);
    }

    Err(format!("Node save failed: {}", &new_node.vid))
}

#[py_sql("
SELECT t.* FROM taxonomies t
  LEFT JOIN node_taxonomies_map ntm ON t.tid = ntm.tid
  WHERE ntm.nid=#{nid}
  AND t.bundle=#{bundle}
  ORDER BY count desc, weight
")]
pub async fn find_node_taxonomies(
    rb: Arc<Rbatis>,
    bundle: &str,
    nid: &i32,
) -> Result<Vec<Taxonomies>, Error> {
 todo!()
}