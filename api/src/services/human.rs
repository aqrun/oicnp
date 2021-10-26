use crate::typings::{GqlResult};
use crate::dbs::StarWars;
use crate::models::{Character};
use crate::utils::query_characters;
use async_graphql::{FieldResult, Error};
use async_graphql::connection::{Connection, EmptyFields};

pub async fn get_human_by_id(db: &StarWars, id: &str) -> GqlResult<Character> {
    if let Some(current_id) = db.human(id) {
        let human = db.chars[current_id].to_human().into();
        Ok(human)
    } else {
        Err(Error::new("human not exist"))
    }
}

pub async fn get_hero(db: &StarWars) -> GqlResult<Character> {
    let hero = db.chars[db.luke].to_human().into();
    Ok(hero)
}

pub async fn get_humans(
    db: &StarWars,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
) -> FieldResult<Connection<usize, Character, EmptyFields, EmptyFields>> {
    let ids = db
        .humans()
        .iter()
        .copied()
        .collect::<Vec<usize>>();

    query_characters(after, before, first, last, &ids)
        .await
        .map(|conn| {
            // 将 usize 索引转为 Character
            conn.map_node(|item| {
                db.chars[item].to_human().into()
            })
        })
}