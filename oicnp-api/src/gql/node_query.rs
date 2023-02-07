use async_graphql::{
    Object, Context, FieldResult,
    connection::{
        Connection, EmptyFields,
        query, Edge,
    },
};
use crate::models::Nodes;
use crate::typings::{
    NodeBundle,
    DetailNode, PagerInfo,
};
use crate::services::{
    find_detail_nodes,
    find_nodes_count,
    find_node_by_nid,
    find_node_by_vid,
    find_nodes_width_target_id,
};
use oicnp_core::{
    DatabaseConnection,
    services::find_nodes,
};

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn nodes(
        &self,
        ctx: &Context<'_>,
        category: Option<String>,
        order_name: Option<String>,
        order_dir: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
        #[graphql(desc = "返回指定ID相关的列表")]
        target_nid: Option<i32>,
    ) -> FieldResult<Connection<i32, DetailNode, PagerInfo, EmptyFields>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let bundle = NodeBundle::Article.to_string();
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);
        let category = category.unwrap_or(String::from(""));
        let limit = page_size;
        let offset = (page - 1 ) * limit;
        let order_name = order_name.unwrap_or(String::from("created_at"));
        let order_dir = order_dir.unwrap_or(String::from("DESC"));
        let filters: Vec<String> = vec!();

        let mut total_count = 0;
        let mut data: Vec<DetailNode> = vec![];

        println!("-----res2 start---");
        let res2 = find_nodes(
            db,
            &bundle,
            &category,
            &filters,
            &order_name,
            &order_dir,
            offset,
            limit
        ).await;

        println!("{:?}------res2", res2);

        let res = match target_nid {
            Some(target_nid) => find_nodes_width_target_id(
                db,
                &bundle,
                &category,
                &filters,
                &order_name,
                &order_dir,
                &limit,
                &target_nid
            ).await,
            _ => find_detail_nodes(
                db,
                &bundle,
                &category,
                &filters,
                &order_name,
                &order_dir,
                &offset,
                &limit
            ).await,
        };

        // 查询数据列表
        if let Ok(res) = res {
            data = res;
        }

        // 获取当前筛选条件对应的数据总数
        if let Ok(res) = find_nodes_count(
            db,
            &bundle,
            &category,
        ).await {
            total_count = res.count;
        }

        let page_info = PagerInfo {
            page,
            page_size,
            total_count
        };

        query(
            None, None, None, None,
            |_after, _before, _first, _last| async move {
                let mut connection = Connection::with_additional_fields(
                    false, false, page_info
                );
                connection.edges.extend(
                    data
                        .iter()
                        .map(|item| Edge::new(item.nid, item.clone()))
                );
                Ok::<_, async_graphql::Error>(connection)
            }
        ).await
    }

    async fn node(
        &self,
        ctx: &Context<'_>,
        bundle: String,
        nid: Option<i32>,
        vid: Option<String>
    ) -> Result<Nodes, String> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let bundle_data = NodeBundle::from(bundle.as_str());
        let mut real_vid = String::from("");
        let mut real_nid = 0;

        if let Some(vid) = &vid {
            real_vid = String::from(vid);

            let res = find_node_by_vid(
                db, &vid, &bundle_data
            ).await;

            if let Ok(res) = res {
                return Ok(res);
            }
        }

        if let Some(nid) = nid {
            real_nid = nid;

            let res = find_node_by_nid(
                db, nid, &bundle_data
            ).await;

            if let Ok(res) = res {
                return Ok(res);
            }
        }

        Err(format!(
            "Node not exist width data: nid[{}], vid[{}]",
            real_nid, real_vid
        ))
    }
}