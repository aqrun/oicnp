use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, ResolveInfo, NextResolve},
    ServerResult, Value,
};
use std::sync::Arc;

///
/// Response extension
/// 错误信息格式化处理扩展
/// 返回需要的标准错误信息格式
///
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
pub struct Resp;

impl ExtensionFactory for Resp {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(RespExtension)
    }
}

struct RespExtension;

#[async_trait::async_trait]
impl Extension for RespExtension {
    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        let name = String::from(info.name);
        let result = next.run(ctx, info).await;

        // let names = vec!("__schema", "directives");
        // if name.as_str().eq("nodes") || name.as_str().eq("updateUser") {
        //     println!("intl;;;;{:?}", name);
        //     println!("res----{:?}", &result);
        // }

        // 错误扩展中包括 自定义 code message时直接返回标准错误类型
        if let Err(err) = &result {
            if let Some(ext) = &err.extensions {
                let mut code = String::from("");
                let mut msg = String::from("");

                if let Some(item) = ext.get("code") {
                    code = item.to_string().replace("\"", "");
                }
                if let Some(item) = ext.get("message") {
                    msg = item.to_string().replace("\"", "");
                }
                if msg.as_str().is_empty() {
                    msg = String::from(err.message.as_str());
                }

                if !code.as_str().is_empty() {
                    let json = serde_json::json!({
                        "code": code,
                        "message": msg,
                        "is_success": false,
                        "locations": &err.locations,
                        "path": &err.path,
                    });

                    let val = async_graphql::Value::from_json(json).unwrap();
                    return Ok(Some(val));
                }
            }
        }

        result
    }
}



