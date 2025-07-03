use std::collections::HashMap;
use crate::models::permissions::PermissionTreeItem;
use crate::entities::prelude::*;
use loco_rs::{prelude::*, cache::Cache};

///
/// 参考示例
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7da973c6d740c895e57dd424ebcf0a35
/// 
pub fn build_permission_tree(list: Vec<PermissionTreeItem>) -> Vec<PermissionTreeItem> {
    let mut roots: Vec<PermissionTreeItem> = Vec::new();
    let mut child_map: HashMap<i64, Vec<PermissionTreeItem>> = HashMap::new();


    for node in list {
        if node.parent_id > 0 {
            if let Some(child_nodes) = child_map.get_mut(&node.parent_id) {
                child_nodes.push(node);
            } else {
                child_map.insert(node.parent_id, vec![node]);
            }
        } else {
            roots.push(node);
        }
    }

    let mut tree_roots = Vec::new();

    for node in roots {
        tree_roots.push(into_tree_node(node, &mut child_map));
    }

    // into_tree_node(root, &mut child_map)
    tree_roots
}

fn into_tree_node (
    node: PermissionTreeItem,
    mut child_map: &mut HashMap<i64, Vec<PermissionTreeItem>>,
) -> PermissionTreeItem {
    let mut children = Vec::new();

    if let Some((_id, child_nodes)) = child_map.remove_entry(&node.id) {
        for child_node in child_nodes {
            children.push(into_tree_node(child_node, &mut child_map));
        }
    }

    let mut new_node = node;

    if !children.is_empty() {
        new_node.children = Some(children);
    }

    new_node
}

///
/// 检测是否为公共API
/// 
pub async fn check_is_public_api(
    db: &DatabaseConnection,
    cache: &Cache,
    api: &str,
) -> bool {
    let cache_key = "public_api_list";

    let is_exist = cache.contains_key(cache_key).await.unwrap_or(false);

    if is_exist {
        let str_public_apis = cache.get(cache_key)
            .await
            .unwrap_or(Some(String::from("")))
            .unwrap_or(String::from(""));
        let apis = str_public_apis.split(",").collect::<Vec<&str>>();
        return apis.contains(&api);
    }

    let permissions = PermissionModel::get_public_permissions(db).await.unwrap();
    let public_apis = permissions.into_iter()
        .map(|p| p.api)
        .collect::<Vec<String>>();
    let apis_str = public_apis.join(",");

    let _ = cache.insert(cache_key, apis_str.as_str()).await;

    public_apis.contains(&String::from(api))
}