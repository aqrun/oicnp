use async_graphql::ErrorExtensions;

///
/// 标准错误返回
///
pub fn oic_err(code: &str, msg: &str) -> async_graphql::Error {
    let err = async_graphql::Error::new(msg)
        .extend_with(|_, e| {
            e.set("code", code);
        });
    err
}