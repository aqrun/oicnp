use anyhow::{anyhow, Result};
use crate::models::{Node, NodeBody, NewNode, Taxonomies, DetailNode, NodeCount,
    NodeTaxonomiesMap,
};
use crate::typings::{
    BodyFormat, NodeBundle, Count, oic_usize,
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
use chrono::prelude::*;

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
    let pager = query.paginate(db, limit as oic_usize);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(offset as oic_usize).await?;

    println!("----1111111111--------{:?} total:{:?}  taotal_page: {:?}", list, total, total_pages);
    info!("{:?}", list);
    Err(anyhow!(""))
}

pub async fn find_nodes_count(
    db: &DatabaseConnection,
    bundle: &str,
    category: &str,
) -> Result<i32> {
    let mut q: Select<CmsNodes> = CmsNodes::find()
        .select_only()
        .column(cms_nodes::Column::Nid)
        .join(
            JoinType::LeftJoin,
            CmsNodes::belongs_to(CmsNodeTaxonomiesMap)
                .from(cms_nodes::Column::Nid)
                .to(cms_node_taxonomies_map::Column::Nid)
                .into()
        ).join(
            JoinType::LeftJoin,
            CmsNodeTaxonomiesMap::belongs_to(CmsTaxonomies)
                .from(cms_node_taxonomies_map::Column::Tid)
                .to(cms_taxonomies::Column::Tid)
                .into()
        ).filter(
            Condition::all()
                .add(cms_nodes::Column::Deleted.eq("0"))
                .add(cms_nodes::Column::Bundle.eq(bundle))
                .add(cms_taxonomies::Column::Name.eq(category))
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


pub async fn find_node_by_vid(db: &DatabaseConnection, vid: &str, bundle: &NodeBundle) -> Result<Node> {
    let mut q = CmsNodes::find();
    q = q.filter(cms_nodes::Column::Vid.eq(vid));
    q = q.filter(cms_nodes::Column::Bundle.eq(bundle.to_string()));
    q = q.filter(cms_nodes::Column::Deleted.eq("0"));

    let res = q.into_model::<Node>().one(db).await?;

    if let Some(node) = res {
        return Ok(node);
    }

    Err(anyhow!("Node not exist: {}", vid))
}

pub async fn find_node_by_nid(db: &DatabaseConnection, nid: &str, bundle: &NodeBundle) -> Result<Node> {
    let mut q = CmsNodes::find();
    q = q.filter(cms_nodes::Column::Nid.eq(nid));
    q = q.filter(cms_nodes::Column::Bundle.eq(bundle.to_string()));
    q = q.filter(cms_nodes::Column::Deleted.eq("0"));

    let res = q.into_model::<Node>().one(db).await?;

    if let Some(node) = res {
        return Ok(node);
    }
    Err(anyhow!("Node not exist: {}", 1))
}

pub async fn find_node_body(db: &DatabaseConnection, nid: &str) -> Result<NodeBody> {
    let mut q = CmsNodeBody::find();
    q = q.filter(cms_node_body::Column::Nid.eq(nid));
    
    let res = q.into_model::<NodeBody>().one(db).await?;

    if let Some(node_body) = res {
        return Ok(node_body);
    }

    Err(anyhow!("NodeBody not exist: {}", 1))
}

pub async fn save_node_content(
    db: &DatabaseConnection,
    nid: &str,
    body: &str,
    body_format: BodyFormat,
    summary: &str,
) -> Result<String> {
    let node_body = cms_node_body::ActiveModel {
        nid: Set(format!("{}", nid)),
        summary: Set(Some(String::from(summary))),
        summary_format: Set(Some(body_format.to_string())),
        body: Set(Some(String::from(body))),
        body_format: Set(Some(body_format.to_string())),
        ..Default::default()
    };

    let res: cms_node_body::Model = match node_body.insert(db).await {
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

    let node = cms_nodes::ActiveModel {
        nid: Set(uuid()),
        vid: Set(Some(String::from(&new_node.vid))),
        bundle: Set(Some(String::from(&new_node.bundle))),
        title: Set(Some(String::from(&new_node.title))),
        viewed: Set(Some(0)),
        deleted: Set(Some("0".to_owned())),
        published_at: Set(new_node.published_at),
        created_by: Set(Some(String::from(&new_node.created_by))),
        updated_by: Set(Some(String::from(&new_node.updated_by))),
        created_at: Set(new_node.created_at),
        updated_at: Set(Some(new_node.updated_at)),
        deleted_at: Set(None),
        ..Default::default()
    };

    let node: cms_nodes::Model = node.insert(db).await?;

    let created_by = node.created_by.unwrap_or("".to_string());

    let data = Node {
        nid: node.nid,
        vid: node.vid.unwrap(),
        bundle: node.bundle.unwrap_or("".to_string()),
        title: node.title.unwrap(),
        viewed: node.viewed.unwrap(),
        deleted: node.deleted.unwrap(),
        published_at: node.published_at,
        created_at: node.created_at,
        created_by: created_by.to_string(),
        updated_at: node.updated_at.unwrap_or(Local::now().naive_local()),
        updated_by: node.updated_by.unwrap().parse().unwrap(),
        deleted_at: node.deleted_at,
    };
    Ok(data)
}

pub async fn save_node_taxonomies_map(
    db: &DatabaseConnection,
    bundle: &str,
    nid: &str,
    tid: &str
) -> Result<NodeTaxonomiesMap> {
    let n = cms_node_taxonomies_map::ActiveModel {
        bundle: Set(Some(String::from(bundle))),
        nid: Set(String::from(nid)),
        tid: Set(String::from(tid)),
    };
    let res = n.insert(db).await?;
    let data = NodeTaxonomiesMap::from_model(&res);
    Ok(data)
}

pub async fn find_node_taxonomies(
    db: &DatabaseConnection,
    nid: &str,
) -> Result<Vec<Taxonomies>> {
    let q = CmsTaxonomies::find()
        .join(
            JoinType::LeftJoin,
            CmsTaxonomies::belongs_to(CmsNodeTaxonomiesMap)
                .from(cms_taxonomies::Column::Tid)
                .to(cms_node_taxonomies_map::Column::Tid)
                .into()
        )
        .filter(
            Condition::all()
                .add(cms_node_taxonomies_map::Column::Nid.eq(nid))
                // .add(cms_node_taxonomies_map::Column::Bundle.eq(bundle))
        );

    // let data = // q.into_model::<Node>()
    //     a.build(DbBackend::Postgres)
    //         .to_string();
    let res = q.into_model::<Taxonomies>().all(db).await?;
    return Ok(res);
    // println!("----{:?}", res);
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