use std::collections::HashMap;
use crate::{
    entities::prelude::*,
    utils::catch_err,
    RequestParamsUpdater,
    ModelCrudHandler,
    models::tags::CreateTagReqParams,
};
use loco_rs::prelude::*;
use sea_orm::{
    prelude::*,
    ActiveValue::NotSet,
    IntoActiveModel,
    QueryOrder,
    JoinType,
    QuerySelect,
    sea_query::Alias,
    Order,
};
use validator::Validate;
use super::{
    CreateNodeReqParams,
    NodeFilters,
    UpdateNodeReqParams,
    DeleteNodeReqParams,
    NodeDetailModel,
};

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
            let _ = Self::create(db, item).await?;
        }

        Ok(String::from("批量node添加完成"))
    }

    /// 创建 node
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut node = NodeActiveModel {
            ..Default::default()
        };

        params.update(&mut node);
        params.update_by_create(&mut node);

        if let Some(x) = &params.created_by_username {
            let user = UserModel::find_by_username(db, x).await?;
            node.created_by = Set(user.uid);
        }

        let node_model = node.insert(db).await?;

        if let Some(x) = &params.category_vids {
            Self::assign_categories(db, node_model.nid, x.as_slice()).await?;
        }

        if let Some(x) = &params.tag_vids {
            Self::assign_tags(db, node_model.nid, x.as_slice()).await?;
        }

        Self::save_content(db, node_model.nid, params).await?;

        Ok(node_model.nid)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let nid = params.nid.unwrap_or(0);

        if nid <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", nid).into()));
        }

        let mut node = Self::find_by_id(db, nid)
            .await?
            .into_active_model();

        params.update(&mut node);
        node.uuid = NotSet;
    
        let node = node.update(db).await?;

        Ok(node.nid)
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

    /// 批量获取多个node的分类
    pub async fn find_multi_nodes_categories(
        db: &DatabaseConnection,
        nids: &[i64],
    ) -> ModelResult<Vec<(i64, CategoryModel)>> {
        if nids.is_empty() {
            return Ok(Vec::new());
        }

        let categories = CategoryEntity::find()
            .left_join(NodeCategoriesMapEntity)
            .filter(NodeCategoriesMapColumn::Nid.is_in(nids.to_vec()))
            .select_also(NodeCategoriesMapEntity)
            .all(db)
            .await?;

        let mut result = Vec::new();
        for (category, map) in categories {
            if let Some(map) = map {
                result.push((map.nid, category));
            }
        }

        Ok(result)
    }

    /// 批量获取多个node的标签
    pub async fn find_multi_nodes_tags(
        db: &DatabaseConnection,
        nids: &[i64],
    ) -> ModelResult<Vec<(i64, TagModel)>> {
        if nids.is_empty() {
            return Ok(Vec::new());
        }

        let tags = TagEntity::find()
            .left_join(NodeTagsMapEntity)
            .filter(NodeTagsMapColumn::Nid.is_in(nids.to_vec()))
            .select_also(NodeTagsMapEntity)
            .all(db)
            .await?;

        let mut result = Vec::new();
        for (tag, map) in tags {
            if let Some(map) = map {
                result.push((map.nid, tag));
            }
        }

        Ok(result)
    }

    /// 批量获取多个node的内容
    pub async fn find_multi_nodes_body(
        db: &DatabaseConnection,
        nids: &[i64],
    ) -> ModelResult<Vec<(i64, NodeBodyModel)>> {
        if nids.is_empty() {
            return Ok(Vec::new());
        }

        let node_bodies = NodeBodyEntity::find()
            .filter(NodeBodyColumn::Nid.is_in(nids.to_vec()))
            .all(db)
            .await?;

        let result = node_bodies.into_iter().map(|body| (body.nid, body)).collect();
        Ok(result)
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

    /// 根据vid创建或更新
    pub async fn upsert_by_vid(db: &DatabaseConnection, params: &CreateNodeReqParams) -> ModelResult<String> {
        let mut vid = String::from("");

        if let Some(x) = &params.vid {
            vid = String::from(x);
        }

        if vid.is_empty() {
            return Err(ModelError::Message(format!("vid为空,vid: {}", vid).into()));
        }

        match Self::find_by_vid(db, vid.as_str()).await {
            Ok(old_node) => {
                let mut new_params = params.clone();
                new_params.nid = Some(old_node.nid);

                Self::update(db, &new_params).await?;
               
                Ok(old_node.vid)
            }
            Err(_) => {
                let _ = Self::create(db, params).await?;
                Ok(vid)
            }
        }
    }

    /**
     * 获取node详细列表
     */
    pub async fn find_node_list(
        db: &DatabaseConnection,
        params: &NodeFilters,
    ) -> ModelResult<(Vec<NodeDetailModel>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order_by_str = params.get_order_by();

        // 设置排序字段和顺序
        let mut order = params.get_order();
        order = Order::Desc;
        let order_by = match order_by_str.as_str() {
            "title" => NodeColumn::Title,
            "updated_at" => NodeColumn::UpdatedAt,
            "viewed" => NodeColumn::Viewed,
            _ => NodeColumn::CreatedAt, // 默认按创建时间
        };

        let mut q = NodeEntity::find()
            .select_only()
            .columns([
                NodeColumn::Nid,
                NodeColumn::Uuid,
                NodeColumn::Vid,
                NodeColumn::Bundle,
                NodeColumn::Title,
                NodeColumn::Viewed,
                NodeColumn::Deleted,
                NodeColumn::PublishedAt,
                NodeColumn::CreatedBy,
                NodeColumn::UpdatedBy,
                NodeColumn::CreatedAt,
                NodeColumn::UpdatedAt,
                NodeColumn::DeletedAt,
            ])
            // 指定关联表字段
            .column_as(NodeBodyColumn::Summary, "summary")
            .column_as(NodeBodyColumn::SummaryFormat, "summary_format")
            // .column_as(NodeBodyColumn::Body, "body")
            // .column_as(NodeBodyColumn::BodyFormat, "body_format")
            .column_as(
                Expr::col((Alias::new("cu"), UserColumn::Uid)),
                "author_uid"
            )
            .column_as(
                Expr::col((Alias::new("cu"), UserColumn::Username)),
                "author_username"
            )
            .column_as(
                Expr::col((Alias::new("cu"), UserColumn::Nickname)),
                "author_nickname"
            )
            .column_as(
                Expr::col((Alias::new("uu"), UserColumn::Username)),
                "updated_by_username"
            )
            .column_as(
                Expr::col((Alias::new("uu"), UserColumn::Nickname)),
                "updated_by_nickname"
            )
            .join(
                JoinType::LeftJoin,
                NodeEntity::belongs_to(NodeBodyEntity)
                    .from(NodeColumn::Nid)
                    .to(NodeBodyColumn::Nid)
                    .into()
            )
            // 关联用户表 指定别名 cu 创建者信息
            .join_as(
                JoinType::LeftJoin,
                NodeEntity::belongs_to(UserEntity)
                    .from(NodeColumn::CreatedBy)
                    .to(UserColumn::Uid)
                    .into(),
                Alias::new("cu"),
            )
            // 关联用户表 指定别名 uu 更新者信息
            .join_as(
                JoinType::LeftJoin,
                NodeEntity::belongs_to(UserEntity)
                    .from(NodeColumn::UpdatedBy)
                    .to(UserColumn::Uid)
                    .into(),
                Alias::new("uu"),
            )
            ;

        // 应用过滤条件
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

        if let Some(x) = &params.bundle {
            if !x.is_empty() {
                q = q.filter(NodeColumn::Bundle.eq(x));
            }
        }

        if let Some(x) = &params.vid {
            if !x.is_empty() {
                q = q.filter(NodeColumn::Vid.eq(x));
            }
        }

        if let Some(x) = &params.deleted {
            if !x.is_empty() {
                q = q.filter(NodeColumn::Deleted.eq(x));
            }
        }

        if let Some(x) = params.created_by {
            if x > 0 {
                q = q.filter(NodeColumn::CreatedBy.eq(x));
            }
        }

        // 按分类查询
        if let Some(category_vids) = &params.category_vids {
            if !category_vids.is_empty() {
                q = q
                    .inner_join(NodeCategoriesMapEntity)
                    .inner_join(CategoryEntity)
                    .filter(CategoryColumn::CatVid.is_in(category_vids.clone()));
            }
        }

        if let Some(category_ids) = &params.category_ids {
            if !category_ids.is_empty() {
                q = q
                    .inner_join(NodeCategoriesMapEntity)
                    .filter(NodeCategoriesMapColumn::CatId.is_in(category_ids.clone()));
            }
        }

        // 按标签查询
        if let Some(tag_vids) = &params.tag_vids {
            if !tag_vids.is_empty() {
                // 先通过标签表获取对应的 tag_ids
                let tag_ids: Vec<i64> = TagEntity::find()
                    .filter(TagColumn::TagVid.is_in(tag_vids.clone()))
                    .select_only()
                    .column(TagColumn::TagId)
                    .into_tuple()
                    .all(db)
                    .await?;
                
                if !tag_ids.is_empty() {
                    q = q
                        .inner_join(NodeTagsMapEntity)
                        .filter(NodeTagsMapColumn::TagId.is_in(tag_ids));
                }
            }
        }

        if let Some(tag_ids) = &params.tag_ids {
            if !tag_ids.is_empty() {
                q = q
                    .inner_join(NodeTagsMapEntity)
                    .filter(NodeTagsMapColumn::TagId.is_in(tag_ids.clone()));
            }
        }
        
        let total = q.clone().count(db).await?;
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let nodes: Vec<NodeModel> = pager.fetch_page(page - 1).await?;

        if nodes.is_empty() {
            return Ok((Vec::new(), total));
        }

        // 收集所有 nid
        let nids: Vec<i64> = nodes.iter().map(|node| node.nid).collect();

        // 批量查询所有关联数据
        let (node_bodies, categories, tags) = tokio::try_join!(
            Self::find_multi_nodes_body(db, &nids),
            Self::find_multi_nodes_categories(db, &nids),
            Self::find_multi_nodes_tags(db, &nids)
        )?;

        // 将关联数据按 nid 分组，使用 HashMap 提高查找效率
        
        let body_map: HashMap<i64, NodeBodyModel> = node_bodies.into_iter().collect();
        
        let mut categories_map: HashMap<i64, Vec<CategoryModel>> = HashMap::new();
        for (nid, category) in categories {
            categories_map.entry(nid).or_insert_with(Vec::new).push(category);
        }
        
        let mut tags_map: HashMap<i64, Vec<TagModel>> = HashMap::new();
        for (nid, tag) in tags {
            tags_map.entry(nid).or_insert_with(Vec::new).push(tag);
        }

        // 转换为 NodeDetailModel
        let mut result = Vec::new();
        for node in nodes {
            let nid = node.nid;
            
            // 获取 node_body，如果没有则使用默认值
            let node_body = body_map.get(&nid).cloned().unwrap_or_default();

            // 获取分类和标签，如果没有则使用空数组
            let node_categories = categories_map.get(&nid).cloned().unwrap_or_default();
            let node_tags = tags_map.get(&nid).cloned().unwrap_or_default();

            let node_detail = NodeDetailModel {
                nid: node.nid,
                vid: node.vid,
                uuid: node.uuid,
                bundle: node.bundle,
                title: node.title,
                viewed: node.viewed,
                deleted: node.deleted,
                published_at: node.published_at,
                created_by: node.created_by,
                updated_by: node.updated_by,
                created_at: node.created_at,
                updated_at: node.updated_at,
                deleted_at: node.deleted_at,
                summary: node_body.summary,
                summary_format: node_body.summary_format,
                // body: Some(node_body.body),
                // body_format: Some(node_body.body_format),
                author_uid: Some(node.created_by),
                author_username: None, // 这些需要从用户表查询
                author_nickname: None,
                updated_by_username: None,
                updated_by_nickname: None,
                categories: node_categories,
                tags: node_tags,
                ..Default::default()
            };

            result.push(node_detail);
        }

        Ok((result, total))
    }
}