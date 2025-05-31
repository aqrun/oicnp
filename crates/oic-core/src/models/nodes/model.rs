use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
    models::tags::CreateTagReqParams,
};
use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder};
use validator::Validate;
use super::{CreateNodeReqParams, NodeFilters, UpdateNodeReqParams, DeleteNodeReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for NodeActiveModel {}

#[async_trait::async_trait]
impl ModelCrudHandler for NodeModel {
    type DataModel = Self;
    type FilterParams = NodeFilters;
    type CreateReqParams = CreateNodeReqParams;
    type UpdateReqParams = UpdateNodeReqParams;
    type DeleteReqParams = DeleteNodeReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = NodeEntity::find()
            .filter(NodeColumn::Nid.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self> {
        if vid.is_empty() {
            return Err(ModelError::Any(format!("vid为空: {}", vid).into()));
        }

        let item = NodeEntity::find()
            .filter(NodeColumn::Vid.eq(vid))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在, vid: {}", vid).into())
        })
    }

    ////
    /// 获取node列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
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

        if let Some(x) = &params.title {
            if !x.is_empty() {
                q = q.filter(NodeColumn::Title.contains(x));
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

        Ok((list, total))
    }

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        catch_err(params.validate())?;

        for item in params.iter() {
            let mut node = NodeActiveModel {
                ..Default::default()
            };
    
            item.update(&mut node);
            item.update_by_create(&mut node);

            if let Some(x) = &item.created_by_username {
                let user = UserModel::find_by_username(db, x).await?;
                node.created_by = Set(user.uid);
            }

            let node_model = node.insert(db).await?;

            if let Some(x) = &item.category_vids {
                Self::assign_categories(db, node_model.nid, x.as_slice()).await?;
            }

            if let Some(x) = &item.tag_vids {
                Self::assign_tags(db, node_model.nid, x.as_slice()).await?;
            }

            Self::save_content(db, node_model.nid, item).await?;
        }

        Ok(String::from("批量node添加完成"))
    }

    /// 创建 node
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = NodeActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.nid)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
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
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
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

impl NodeModel {
    /// 指定分类
    pub async fn assign_categories(
        db: &DatabaseConnection,
        nid: i64,
        category_vids: &[String],
    ) -> ModelResult<()> {
        let categories = CategoryEntity::find()
            .all(db)
            .await?;

        for vid in category_vids.iter() {
            let category = categories.iter().find(|c| c.cat_vid.eq(vid));

            if let Some(category) = category {
                let node_cat = NodeCategoriesMapActiveModel {
                    bundle: Set(String::from("post")),
                    nid: Set(nid),
                    cat_id: Set(category.cat_id),
                };

                node_cat.insert(db).await?;
            }
        }

        Ok(())
    }

    /// 指定标签
    pub async fn assign_tags(
        db: &DatabaseConnection,
        nid: i64,
        tag_vids: &[String],
    ) -> ModelResult<()> {
        let tag_params = tag_vids.iter().map(|vid| CreateTagReqParams {
            tag_vid: Some(vid.to_string()),
            tag_name: Some(vid.to_string()),
            ..Default::default()
        }).collect::<Vec<CreateTagReqParams>>();
        TagModel::create_multi(db, tag_params.as_slice()).await?;

        for vid in tag_vids.iter() {
            let tag = TagEntity::find()
                .filter(TagColumn::TagVid.eq(vid))
                .one(db)
                .await?;

            if let Some(tag) = tag {
                let node_tag = NodeTagsMapActiveModel {
                    bundle: Set(String::from("post")),
                    nid: Set(nid),
                    tag_id: Set(tag.tag_id),
                };

                node_tag.insert(db).await?;
                // 更新标签计数
                TagModel::update_count_by_id(db, tag.tag_id).await?;
            }
        }

        Ok(())
    }

    /// 保存内容
    pub async fn save_content(db: &DatabaseConnection, nid: i64, params: &CreateNodeReqParams) -> ModelResult<()> {
        let mut node_body = NodeBodyActiveModel {
            nid: Set(nid),
            ..Default::default()
        };

        if let Some(x) = &params.summary {
            node_body.summary = Set(x.to_string());
        }

        if let Some(x) = &params.summary_format {
            node_body.summary_format = Set(x.to_string());
        }

        if let Some(x) = &params.body {
            node_body.body = Set(x.to_string());
        }

        if let Some(x) = &params.body_format {
            node_body.body_format = Set(x.to_string());
        }

        node_body.insert(db).await?;

        Ok(())
    }

    /// 根据nid获取分类
    pub async fn find_categories(db: &DatabaseConnection, nid: i64) -> ModelResult<Vec<CategoryModel>> {
        let categories = CategoryEntity::find()
            .left_join(NodeCategoriesMapEntity)
            .filter(NodeCategoriesMapColumn::Nid.eq(nid))
            .all(db)
            .await?;

        Ok(categories)
    }

    /// 根据nid获取标签
    pub async fn find_tags(db: &DatabaseConnection, nid: i64) -> ModelResult<Vec<TagModel>> {
        let tags = TagEntity::find()
            .left_join(NodeTagsMapEntity)
            .filter(NodeTagsMapColumn::Nid.eq(nid))
            .all(db)
            .await?;

        Ok(tags)
    }

    pub async fn find_node_body(db: &DatabaseConnection, nid: i64) -> ModelResult<NodeBodyModel> {
        let node_body = NodeBodyEntity::find()
            .filter(NodeBodyColumn::Nid.eq(nid))
            .one(db)
            .await?;

        node_body.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", nid).into())
        })
    }

}