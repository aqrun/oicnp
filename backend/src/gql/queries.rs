use async_graphql::{Object, Context, FieldResult};
use crate::typings::{GqlResult, Episode};
use async_graphql::connection::{Connection, EmptyFields};
use crate::dbs::StarWars;
use crate::services;
use crate::models::{Character};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
        #[graphql(
            desc = "无值返回所有系列的角色，有值返回指定的系列角色"
        )]
        episode: Option<Episode>,
    ) -> GqlResult<Character> {
        let db = ctx.data_unchecked::<StarWars>();

        if let Some(episode) = episode {
            if episode == Episode::Empire {
                services::human::get_hero(db).await
            } else {
                services::droid::get_hero(db).await
            }
        } else {
            services::human::get_hero(db).await
        }
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "人类ID")]
        id: String,
    ) -> GqlResult<Character> {
        let db = ctx.data_unchecked::<StarWars>();
        services::human::get_human_by_id(db, &id).await
    }

    async fn humans(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Character, EmptyFields, EmptyFields>> {
        let db = ctx.data_unchecked::<StarWars>();
        services::human::get_humans(db, after, before, first, last).await
    }
}