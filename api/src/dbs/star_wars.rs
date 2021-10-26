use std::collections::HashMap;
use slab::Slab;
use crate::typings::{Episode};
use crate::models::{StarWarsChar};

///
/// 星球大战所有数据
///
pub struct StarWars {
    // 卢克·天行者
    pub luke: usize,
    // R2-D2
    pub artoo: usize,
    // 所有角色
    pub chars: Slab<StarWarsChar>,
    // 人类数据
    pub human_data: HashMap<&'static str, usize>,
    // 机器人数据
    pub droid_data: HashMap<&'static str, usize>,
}

impl StarWars {
    #[allow(clippy::new_without_default)]
    pub fn new() {
        let mut chars = Slab::new();
/*
        let luke = chars.insert(StarWarsChar {
            id: "1000",
            name: "卢克·天行者",
            en_name: "Luke Skywalker",
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("塔图因星(Tatooine)"),
            primary_function: None,
        });

        // 卢克
        let vader = chars.insert(StarWarsChar {
            id: "1001",
            name: "卢克·天行者",
            en_name: "Luke Skywalker",
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("塔图因星(Tatooine)"),
            primary_function: None,
        });

        // 汉
        let han = chars.insert(StarWarsChar {
            id: "1002",
            name: "汉·索罗",
            en_name: "Han Solo",
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: None,
        });

        // 莉亚
        let leia = chars.insert(StarWarsChar {
            id: "1003",
            name: "莉亚·欧嘉纳",
            en_name: "Leia Organa",
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: Some("奥德朗星(Alderaa)"),
            primary_function: None,
        });

        // 塔金
        let tarkin = chars.insert(StarWarsChar {
            id: "1004",
            name: "威尔赫夫·塔金",
            en_name: "Wilhuff Tarkin",
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: None,
        });

        // 3po
        let threepio = chars.insert(StarWarsChar {
            id: "2000",
            name: "C-3PO",
            en_name: "C-3PO",
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: Some("礼仪机器人(Protocol)"),
        });

        // r2
        let artoo = chars.insert(StarWarsChar {
            id: "2001",
            name: "R2-D2",
            en_name: "R2-D2",
            friends: vec![],
            appears_in: vec![Episode::Empire, Episode::NewHope, Episode::Jedi],
            home_planet: None,
            primary_function: Some("宇航技工机器人(Astromech)"),
        });
        Luke Skywalker,Han Solo,Leia Organa,Wilhuff Tarkin,C-3PO,R2-D2
        // 指定朋友关系
        chars[luke].friends = vec![han, leia, threepio, artoo];
        chars[vader].friends = vec![tarkin];
        chars[han].friends = vec![luke, leia, artoo];
        chars[leia].friends = vec![luke, han, threepio, artoo];
        chars[tarkin].friends = vec![vader];
        chars[threepio].friends = vec![luke, han, leia, artoo];
        chars[artoo].friends = vec![luke, han, leia];

        // 人类列表
        let mut human_data = HashMap::new();
        human_data.insert("1000", luke);
        human_data.insert("1001", vader);
        human_data.insert("1002", han);
        human_data.insert("1003", leia);
        human_data.insert("1004", tarkin);

        let mut droid_data = HashMap::new();
        droid_data.insert("2000", threepio);
        droid_data.insert("2001", artoo);

        Self {
            luke,
            artoo,
            chars,
            human_data,
            droid_data,
        }
        */

    }

    pub fn human(&self, id: &str) -> Option<usize> {
        self.human_data.get(id).cloned()
    }

    pub fn droid(&self, id: &str) -> Option<usize> {
        self.droid_data.get(id).cloned()
    }

    pub fn humans(&self) -> Vec<usize> {
        self.human_data.values().cloned().collect()
    }

    pub fn droids(&self) -> Vec<usize> {
        self.droid_data.values().cloned().collect()
    }

}