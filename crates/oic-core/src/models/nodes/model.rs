use crate::{
    entities::prelude::*,
    typings::ListData,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait};
use validator::Validate;
use super::{CreateNodeReqParams, NodeFilters, UpdateNodeReqParams, DeleteNodeReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for NodeActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for NodeModel {
    type CreateReqParams = CreateNodeReqParams;

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        catch_err(params.validate())?;
        
        let txn = db.begin().await?;
        let mut notes: Vec<NodeActiveModel> = Vec::new();

        for item in params.iter() {
            let mut note = NodeActiveModel {
                ..Default::default()
            };
    
            item.update(&mut note);
            item.update_by_create(&mut note);

            notes.push(note);
        }
        
        let _ = NodeEntity::insert_many(notes).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量node添加完成"))
    }
}

impl NodeModel {
    ///
    /// 根据ID查找一个
    /// 
    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = NodeEntity::find()
            .filter(NoteColumn::Id.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ////
    /// 获取node列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: NodeFilters) -> ModelResult<ListData<Self>> {
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
    pub async fn create(db: &DatabaseConnection, params: &CreateNodeReqParams) -> ModelResult<Self> {
        catch_err(params.validate())?;

        let mut item = NodeActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item)
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateNodeReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let nid = params.nid.unwrap_or(0);

        if nid <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", nid).into()));
        }

        let mut item = Self::find_by_id(db, nid)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.nid)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteNodeReqParams) -> ModelResult<i64> {
        let nid = params.nid.unwrap_or(0);

        if nid <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", nid).into()));
        }

        let _res = NodeEntity::delete_by_id(nid)
            .exec(db)
            .await?;

        Ok(nid)
    }
}