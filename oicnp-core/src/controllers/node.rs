use crate::models::Node;
use crate::services::{
    find_node_by_nid, find_node_by_vid, find_nodes,
};
use crate::typings::{GqlResult, JsonResponse};
use crate::utils::oic_err;
use async_graphql::{
    self, Context, Object,
};
use crate::{
    prelude::anyhow::{anyhow, Result},
    typings::{NodeBundle, ListData},
    DatabaseConnection,
};
use serde_json::json;

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn find_node(
        &self,
    ) -> GqlResult<JsonResponse> {
        Ok(JsonResponse::success(json!({})))
    }

    async fn nodes(
        &self,
        ctx: &Context<'_>,
        category: Option<String>,
        order_name: Option<String>,
        order_dir: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> GqlResult<ListData<Node>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);
        let category = category.unwrap_or(String::from(""));
        let order_name = order_name.unwrap_or(String::from("created_at"));
        let order_dir = order_dir.unwrap_or(String::from("DESC"));
        let filters: Vec<String> = vec![];
        let bundle = NodeBundle::Article.to_string();

        // println!("-----res2 start---");
        let res = find_nodes(
            db,
            bundle.as_str(),
            &category,
            &filters,
            &order_name,
            &order_dir,
            page as u64,
            page_size as u64,
        )
        .await;

        // println!("{:?}------res2", res);

        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = err.to_string();
                return Err(oic_err("400", msg.as_str()));
            }
        }
    }

    async fn node(
        &self,
        ctx: &Context<'_>,
        bundle: String,
        nid: Option<i64>,
        vid: Option<String>,
    ) -> Result<Node, String> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let bundle_data = NodeBundle::from(bundle.as_str());
        let mut real_vid = String::from("");

        if let Some(vid) = &vid {
            real_vid = String::from(vid);

            let res = find_node_by_vid(db, &vid, &bundle_data).await;

            if let Ok(res) = res {
                return Ok(res);
            }
        }

        if let Some(nid) = nid {

            let res = find_node_by_nid(db, nid, &bundle_data).await;

            if let Ok(res) = res {
                return Ok(res);
            }
        }

        Err(format!(
            "Node not exist width data: nid[{}], vid[{}]",
            nid.unwrap_or(0), real_vid
        ))
    }
}

#[derive(Default)]
pub struct NodeMutations;

#[Object]
impl NodeMutations {
    async fn create_node(&self, ctx: &Context<'_>, title: String) -> Result<String> {
        // Ok(format!("Node create success {}", title))
        return Err(anyhow!(""))
    }

    async fn update_node(&self, ctx: &Context<'_>, title: String) -> Result<String> {
        // Ok(format!("Node create success {}", title))
        return Err(anyhow!(""))
    }

    async fn delete_node(&self, ctx: &Context<'_>, title: String) -> Result<String> {
        // Ok(format!("Node create success {}", title))
        return Err(anyhow!(""))
    }
}

