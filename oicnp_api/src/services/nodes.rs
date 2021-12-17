use std::sync::Arc;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use crate::models::{
    Nodes, NodeBody, NewNode,
};
use crate::typings::{
    BodyFormat, NodeBundle,
};

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