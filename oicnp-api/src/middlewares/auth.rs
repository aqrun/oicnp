use poem::{Endpoint, IntoResponse, Middleware, Request, Error,
           web::Json, Result, http::StatusCode, Body,
};
use bytes::Bytes;
use async_graphql::{Request as GraphqlRequest, value};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AuthMiddleware;

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let ori_uri_path = req.original_uri().path().to_string();
        let method = req.method().to_string();
        let path_params = req.uri().query().unwrap_or("").to_string();
        let (req_parts, req_body) = req.into_parts();
        let (bytes, body_data) = match get_body_data(req_body).await {
            Err(e) => return Err(e),
            Ok((x, y)) => (x, y),
        };

        // println!("ori_uri_path: {}, method: {}, path_params:{}", ori_uri_path, method, path_params);
        // println!("req_parts: {:?}", &req_parts);
        let mut need_check_auth = true;

        if req_parts.uri.eq("/") && req_parts.method.eq("GET") {
            need_check_auth = false;
        } else {
            let graphql_req = GraphqlRequest::from(value!({body_data.as_str()}));
            let r = Arc::new(Mutex::new(graphql_req));

            let query_source = String::from(r.clone().lock().unwrap().query.as_str());
            let mut operation_name = String::new();

            let r1 = r.clone();
            let r2 = r.clone();
            let mut r3 = r.clone();
            {
                let a = r1.lock().unwrap();
                if let Some(name) = &a.operation_name {
                    operation_name = String::from(name);
                }
            }

            println!("-------------query, -------operation_name:{:?}",
                     // query_source,
                     operation_name,
            );

            {
                println!("------ext: {:?}", r2.lock().unwrap().extensions);
            }

            {
                println!("-------parsed_query: {:?}", r3.lock().unwrap().parsed_query());
            }

        }

        if need_check_auth {
            let data = Json(serde_json::json!({
                "code": "403",
                "message": "No auth",
            }));
            let mut res_data = data.into_response();
            res_data.set_status(StatusCode::FORBIDDEN);
            return Err(Error::from_response(res_data));
        }

        let req = Request::from_parts(req_parts, Body::from(bytes));

        self.ep.call(req).await
    }
}

/// 获取body数据
async fn get_body_data(body: Body) -> Result<(Bytes, String)> {
    let bytes = match body.into_bytes().await {
        Ok(v) => v,
        Err(e) => return Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    };
    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出，可能为blob，binary".to_string())),
    }
}