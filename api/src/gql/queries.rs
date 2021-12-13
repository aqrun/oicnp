use async_graphql::{MergedObject};
use crate::gql::{
    UserQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);