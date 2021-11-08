use async_graphql::{Schema, EmptySubscription};
use crate::gql::{QueryRoot, MutationRoot};
use std::sync::Arc;
use crate::dbs::ConnectionPool;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    >,
    pub connection_pool: Arc<ConnectionPool>,
}