use async_graphql::{Object, Context, MergedObject};
use crate::gql::{
    UserQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);