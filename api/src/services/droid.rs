use crate::dbs::StarWars;
use crate::typings::{GqlResult};
use crate::models::{Character};
use async_graphql::Error;

pub async fn get_droid_by_id(db: &StarWars, id: &str) -> GqlResult<Character> {
    if let Some(current_id) = db.droid(id) {
        let droid = db.chars[current_id].to_droid().into();
        Ok(droid)
    } else {
        Err(Error::new("droid not exist"))
    }
}

pub async fn get_hero(db: &StarWars) -> GqlResult<Character> {
    let hero = db.chars[db.artoo].to_droid().into();
    Ok(hero)
}