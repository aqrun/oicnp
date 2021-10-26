use async_graphql::{Context, Object};
use crate::typings::{Episode};
use crate::dbs::StarWars;
use super::Character;

/// 人类
pub struct Human {
    pub id: &'static str,
    // 姓名
    pub name: &'static str,
    // 英文姓名
    pub en_name: &'static str,
    // 朋友
    pub friends: Vec<usize>,
    // 参与的电影系列
    pub appears_in: Vec<Episode>,
    // 籍贯行星
    pub home_planet: Option<&'static str>,
}

#[Object]
impl Human {
    pub async fn id(&self) -> &str {
        self.id
    }

    pub async fn name(&self) -> &str {
        self.name
    }

    pub async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        let db = ctx.data_unchecked::<StarWars>();
        self.friends
            .iter()
            .map(|id| {
                db.chars[*id].to_droid().into()
            })
            .collect()
    }

    pub async fn appears_in(&self) -> Vec<Episode> {
        self.appears_in.clone()
    }

    pub async fn home_planet(&self) -> Option<&str> {
        self.home_planet
    }

}