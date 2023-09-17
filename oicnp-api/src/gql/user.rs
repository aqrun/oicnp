use crate::gql::GqlResult;
use crate::models::{LoginInfo, Users};
use crate::services;
use crate::utils;
use async_graphql::{Context, Object};
use oicnp_core::{
    entities::cms_nodes,
    prelude::{
        anyhow::{anyhow, Result},
        chrono::prelude::*,
        serde::{Deserialize, Serialize},
    },
    services::auth::create_jwt,
    DatabaseConnection, DateTime,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn create_user(&self, ctx: &Context<'_>) -> GqlResult<String> {
        Ok("".to_string())
    }

    async fn users(&self, ctx: &Context<'_>) -> GqlResult<Vec<Users>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // let res = rb.fetch_list::<Users>().await;

        // if let Ok(res) = res {
        //     return Ok(res);
        // }

        Ok(vec![])
    }

    async fn user(&self, ctx: &Context<'_>, uid: i32) -> Result<Users, String> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // let res = services::find_user_by_id(rb.clone(), uid).await;
        // res
        Err(String::from("test"))
    }
}

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    async fn add(&self) -> GqlResult<usize> {
        Ok(0usize)
    }
}
