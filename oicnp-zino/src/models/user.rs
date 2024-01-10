use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};

#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    DecodeRow,
    Schema,
    ModelAccessor,
    ModelHooks,
    Model,
)]
#[serde(default)]
pub struct User {
    #[schema(primary_key)]
    uid: String,
    #[schema(
        not_null,
        index_type = "text",
        comment = "用户名"
    )]
    username: String,
    #[schema(
        not_null,
        default_value = 0
        comment = "年龄"
    )]
    age: u32,
}

