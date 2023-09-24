use crate::models::{NewNode, NodeBody, Nodes, Taxonomies};
use crate::typings::{BodyFormat, Count, DetailNode, ResListData};
use oicnp_core::{
    entities::cms_nodes,
    prelude::anyhow::{anyhow, Result},
    services as core_services,
    typings::NodeBundle,
    DatabaseConnection,
};

pub async fn find_nodes(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str,  // DESC
    page: u64,
    page_size: u64,
) -> Result<ResListData<DetailNode>> {
    let res = core_services::find_nodes(
        db, bundle, category, filters, order_name, order_dir, page, page_size,
    )
    .await?;

    let data = res
        .data
        .into_iter()
        .map(move |item| {
            return DetailNode {
                data: item,
            };
        })
        .collect::<Vec<DetailNode>>();

    let res_list_data = ResListData {
        data,
        page: res.page,
        page_size: res.page_size,
        total_pages: res.total_pages,
        total_count: res.total_count,
    };
    Ok(res_list_data)
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
pub async fn find_nodes_count(db: &DatabaseConnection, category: &str) -> Result<Count> {
    Err(anyhow!("map not exist"))
}

pub async fn find_node_by_vid(
    db: &DatabaseConnection,
    vid: &str,
    bundle: &NodeBundle,
) -> Result<Nodes> {
    let res = core_services::find_node_by_vid(db, vid, bundle).await;

    if let Ok(node) = res {
        let node_obj = Nodes::from(&node);
        return Ok(node_obj);
    }
    Err(anyhow!("Node not exist: {}", ""))
}

pub async fn find_node_by_nid(
    db: &DatabaseConnection,
    nid: &str,
    bundle: &NodeBundle,
) -> Result<Nodes> {
    let res = core_services::find_node_by_nid(db, nid, bundle).await;

    if let Ok(node) = res {
        let node_obj = Nodes::from(&node);
        return Ok(node_obj);
    }
    Err(anyhow!("Node not exist: {}", 1))
}

pub async fn find_node_body(db: &DatabaseConnection, nid: &str) -> Result<NodeBody> {
    let res = core_services::find_node_body(db, nid).await?;
    let res = NodeBody::from(&res);
    Ok(res)
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

pub async fn create_node(
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

pub async fn find_node_taxonomies(db: &DatabaseConnection, nid: &str) -> Result<Vec<Taxonomies>> {
    let res = core_services::find_node_taxonomies(db, nid).await?;
    let data = res
        .iter()
        .map(|item| Taxonomies::from(item))
        .collect::<Vec<Taxonomies>>();
    Ok(data)
}

///
/// 获取指定 target_id 对应的 相关数据列表
///
pub async fn find_nodes_with_target_id(
    db: &DatabaseConnection,
    category: &str,
    filters: &Vec<String>,
    order_name: &str, // created_at
    order_dir: &str,  // DESC
    limit: &i32,
    target_nid: &i32,
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
