---
title: 'Rust Poem 共享上下文数据到 GraphQL'
description: '根据 单一数据源架构模式，graphql只是接口层，用户授权相关是在业务逻辑层，通常使用Http框架的中间件实现。如果graphql需要消费web框架生成的数据，这时就需要手动数据上下文转换。'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'poem', 'graphql', '中间件']
---

根据 单一数据源架构模式，graphql 只是接口层，用户授权相关是在业务逻辑层，通常使用 Http 框架的中间件实现。
如果 graphql 需要消费 web 框架生成的数据，这时就需要手动数据上下文转换。

## 网站入口 Poem 应用初始化

```rust
// graphql schema
let schema = build_schema().await;
// 请求上下文全局状态数据
let state = State {
      schema,
      // 初始的空数据也可以使用 Option 类型
      req_ctx: init_default_req_ctx(),
};
// poem应用初始化
let app = Route::new()
      // 绑定graphql 路由
      .at(path, get(graphiql).post(graphql))
      // 权限检测中间件
      .with(AuthMiddleware)
      // 当前用户信息和请求上下文数据整理
      .with(CtxMiddleware)
      .data(state);
```

## Poem 中间件数据处理

自定义 Context 中间件，主要对当前用户信息和请求体参数格式化

```rust
/// req上下文注入中间件 同时进行jwt授权验证
pub struct Context;

impl<E: Endpoint> Middleware<E> for Context {
    type Output = ContextEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ContextEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct ContextEndpoint<E> {
    inner: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for ContextEndpoint<E> {
    type Output = E::Output;
    // type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        // Http全局状态数据获取
        let mut state: Option<State> = None;

        {
            if let Some(state_data) = req.extensions().get::<State>() {
                state = Some(state_data.clone());
            }
        }

        // 请求信息ctx注入
        let auth_token = get_request_auth_token(&req);

        // 解析JWT TOKEN
        let login_info= decode_jwt(
            auth_token.as_str(),
            !gql_is_public_query
        ).unwrap();

        // 生成新的请求上下文数据
        let req_ctx = ReqCtx {
            ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
            path,
            path_params,
            method: method.clone(),
            login_info,
            data: body_data.clone(),
            gql_operation_name,
            gql_variables,
            gql_trimmed_query,
            gql_is_public_query,
        };

        // 生成新的 Request对象
        let mut req = Request::from_parts(req_parts, Body::from(bytes));

        if let Some(state) = state {
            // 移除旧的全局State数据
            req.extensions_mut().remove::<State>();
            // 生成新的 State
            let new_state = State {
                schema: state.schema.clone(),
                req_ctx,
            };
            req.extensions_mut().insert(new_state);
        }

        // 开始请求数据
        self.inner.call(req).await
    }
}
```

## 上下文数据注入到 graphql

```rust
#[handler]
pub async fn graphql(
    data: Data<&State>,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = gql_req.0;
    let schema = data.0.schema.clone();

    // 将 poem 中生成请求上下文转入 graphql
    gql_req = gql_req.data(data.0.req_ctx.clone());

    schema.execute(gql_req).await.into()
}
```

## Graphql 中消费数据

通过上面的数据注入，在 Graphql 上文中获取到对应的数据类型

```rust
async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
      // 从graphql 上下文获取对应的数据类型
      let req_ctx = ctx.data_unchecked::<ReqCtx>();

      println!("req-ctx-----{:?}", req_ctx);

      Ok(true)
}
```
