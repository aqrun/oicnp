use std::collections::HashMap;
use crate::entities::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct MenuTreeItem {
    pub id: i32,
    pub mid: String,
    pub pid: String,
    pub path: String,
    pub name: String,
    pub weight: i32,
    pub icon: String,
    pub children: Vec<MenuTreeItem>,
}

pub fn build_menu_tree(menus: Vec<MenuModel>) -> MenuTreeItem {
    let menus: Vec<MenuTreeItem> = menus.into_iter().map(|item| {
        let data = MenuTreeItem {
            id: item.id,
            mid: item.mid,
            pid: item.pid,
            path: item.path,
            name: item.name,
            weight: item.weight,
            icon: item.icon,
            children: Vec::new(),
        };
        return data;
    }).collect::<Vec<MenuTreeItem>>();

    println!("menus----{:?}", menus.clone());

    let mut menu_map: HashMap<String, MenuTreeItem> = HashMap::new();
    
    // Populate the map
    for mut item in menus {
        item.children = Vec::new(); // Initialize children
        menu_map.insert(item.mid.clone(), item);
    }

    // for item in menu_map.values() {
    //     if !item.pid.is_empty() {
    //         if let Some(parent) = menu_map.get_mut(item.pid.as_str()) {
    //             parent.children.push(item.clone());
    //         } 
    //     }
    // }

    let d = MenuTreeItem::default();
    // let a = roots.get(0).unwrap_or(&d);
    // a.clone()
    d
}