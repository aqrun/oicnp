use std::collections::HashMap;
use crate::entities::prelude::*;
use crate::models::menus::MenuTreeItem;

///
/// 参考示例
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7da973c6d740c895e57dd424ebcf0a35
/// 
pub fn build_menu_tree(menus: Vec<MenuModel>) -> MenuTreeItem {
    // let mut roots = Vec::new();
    // 只有一个根节点
    let mut root = MenuModel::default();
    let mut child_map: HashMap<String, Vec<MenuModel>> = HashMap::new();

    for node in menus {
        if !node.pid.is_empty() {
            if let Some(child_nodes) = child_map.get_mut(node.pid.as_str()) {
                child_nodes.push(node);
            } else {
                child_map.insert(String::from(node.pid.as_str()), vec![node]);
            }
        } else {
            // roots.push(node);
            root = node;
        }
    }

    /*
    let mut tree_roots = Vec::new();
    for node in roots {
        tree_roots.push(into_tree_node(node, &mut child_map));
    }
    */

    into_tree_node(root, &mut child_map)
}

fn into_tree_node (
    menu: MenuModel,
    mut child_map: &mut HashMap<String, Vec<MenuModel>>,
) -> MenuTreeItem {
    let mut children = Vec::new();

    if let Some((_id, child_nodes)) = child_map.remove_entry(menu.mid.as_str()) {
        for child_node in child_nodes {
            children.push(into_tree_node(child_node, &mut child_map));
        }
    }

    MenuTreeItem {
        id: menu.id,
        mid: menu.mid,
        pid: menu.pid,
        path: String::from(menu.path.as_str()),
        key: String::from(menu.path.as_str()),
        label: String::from(menu.name.as_str()),
        weight: menu.weight,
        icon: menu.icon,
        children,
    }
}