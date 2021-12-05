use async_graphql::{Schema, EmptySubscription};
use crate::gql::{QueryRoot, MutationRoot};
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    >,
    pub rbatis: Arc<Rbatis>,
}