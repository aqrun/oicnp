use std::collections::HashMap;
use crate::models::menus::MenuTreeItem;

///
/// 参考示例
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7da973c6d740c895e57dd424ebcf0a35
/// 
pub fn build_menu_tree(menus: Vec<MenuTreeItem>) -> Vec<MenuTreeItem> {
    let mut roots = Vec::new();
    let mut child_map: HashMap<i64, Vec<MenuTreeItem>> = HashMap::new();

    for node in menus {
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

    tree_roots
}

fn into_tree_node (
    menu: MenuTreeItem,
    mut child_map: &mut HashMap<i64, Vec<MenuTreeItem>>,
) -> MenuTreeItem {
    let mut children = Vec::new();

    if let Some((_id, child_nodes)) = child_map.remove_entry(&menu.id) {
        for child_node in child_nodes {
            children.push(into_tree_node(child_node, &mut child_map));
        }
    }

    let mut new_node = menu;

    if !children.is_empty() {
        new_node.children = Some(children);
    }

    new_node
}