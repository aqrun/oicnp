use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    typings::ListData,
};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, Set, TransactionTrait};
use serde_json::json;
use validator::Validate;
use super::{CreateNodeReqParams, NodeFilters, UpdateNodeReqParams, DeleteNodeReqParams};
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
impl ActiveModelBehavior for NodeActiveModel {}

impl NodeModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Self> {
        if id < 0 {
            return Err(anyhow!("数据不存在,id: {}", id));
        }

        let item = NodeEntity::find()
            .filter(NoteColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            anyhow!("数据不存在,id: {}", id)
        })
    }

    ////
    /// 获取node列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: NodeFilters) -> Result<ListData<Self>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = NodeEntity::find();

        if let Some(x) = params.nid {
            if x > 0 {
                q = q.filter(NodeColumn::Nid.eq(x));
            }
        }

        if let Some(x) = params.title {
            if !x.is_empty() {
                q = q.filter(NodeColumn::Title.contains(&x));
            }
        }

        let mut order_by = NodeColumn::Nid;

        if order_by_str.eq("title") {
            order_by = NodeColumn::Title;
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        let res = ListData {
            data: list,
            page,
            page_size,
            total,
        };

        Ok(res)
    }

    /// 创建 node
    pub async fn create(db: &DatabaseConnection, params: &CreateNodeReqParams) -> Result<Self> {
        let _ = catch_err(params.validate())?;

        let mut item = NodeActiveModel {
            ..Default::default()
        };

        item.set_from_json(json!(params))?;
        item.created_at = Set(utc_now());
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 批量创建 node
    pub async fn create_multi(db: &DatabaseConnection, params: &[CreateNodeReqParams]) -> Result<String> {
        let _ = catch_err(params.validate())?;
        
        let txn = db.begin().await?;
        let mut notes: Vec<NodeActiveModel> = Vec::new();

        for item in params.iter() {
            match NodeActiveModel::from_json(json!(item)) {
                Ok(mut note) => {
                    note.created_at = Set(utc_now());
                    notes.push(note);
                },
                Err(err) => {
                    txn.rollback().await?;
                    return Err(anyhow!("批量数据有误, {}", err));
                }
            };
        }
        
        let _ = NodeEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量node添加完成"))
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateNodeReqParams) -> Result<i64> {
        let _ = catch_err(params.validate())?;
        let nid = params.nid.unwrap_or(0);

        if nid < 0 {
            return Err(anyhow!("数据不存在,id: {}", nid));
        }

        let mut item = Self::find_by_id(&db, nid)
            .await?
            .into_active_model();

        item.set_from_json(json!(params))?;
        item.updated_at = Set(Some(utc_now()));
    
        let item = item.update(db).await?;

        Ok(item.nid)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteNodeReqParams) -> Result<i64> {
        let nid = params.nid.unwrap_or(0);

        if nid < 0 {
            return Err(anyhow!("数据不存在,id: {}", nid));
        }

        let _res = NodeEntity::delete_by_id(nid)
            .exec(db)
            .await?;

        Ok(nid)
    }
}