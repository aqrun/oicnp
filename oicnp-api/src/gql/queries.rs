use async_graphql::{MergedObject};
use crate::gql::{
    UserQuery, NodeQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, NodeQuery);