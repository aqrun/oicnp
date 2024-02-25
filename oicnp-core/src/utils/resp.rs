use crate::prelude::async_graphql::{self, ErrorExtensions, Error};

///
/// 标准错误返回
///
pub fn oic_err(code: &str, msg: &str) -> Error {
    let err = Error::new(msg)
        .extend_with(|_, e| {
            e.set("code", code);
        });
    err
}