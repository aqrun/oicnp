use std::collections::{HashMap, HashSet};
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
    // sea_query::Alias,
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
        // let mut order = params.get_order();
        let order = Order::Desc;
        let order_by = match order_by_str.as_str() {
            "title" => NodeColumn::Title,
            "updated_at" => NodeColumn::UpdatedAt,
            "viewed" => NodeColumn::Viewed,
            _ => NodeColumn::CreatedAt, // 默认按创建时间
        };

        let mut q = NodeEntity::find()
            // .select_only()
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
            /*
            // 指定关联表字段
            .column_as(NodeBodyColumn::Summary, "summary")
            .column_as(NodeBodyColumn::SummaryFormat, "summary_format")
            .column_as(NodeBodyColumn::Body, "body")
            .column_as(NodeBodyColumn::BodyFormat, "body_format")
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
            */
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

        // 按分类查询 - 合并处理避免重复连接
        let has_category_vids = params.category_vids.as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);
        let has_category_ids = params.category_ids.as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_category_vids || has_category_ids {
            // 使用关系定义连接，避免重复连接
            q = q.join(JoinType::InnerJoin, NodeEntity::belongs_to(NodeCategoriesMapEntity)
                .from(NodeColumn::Nid)
                .to(NodeCategoriesMapColumn::Nid)
                .into());
            
            // 如果使用 category_vids，需要连接 CategoryEntity
            if has_category_vids {
                if let Some(category_vids) = &params.category_vids {
                    let vids = category_vids.split(",").collect::<Vec<&str>>();
                    q = q.join(JoinType::InnerJoin, NodeCategoriesMapEntity::belongs_to(CategoryEntity)
                        .from(NodeCategoriesMapColumn::CatId)
                        .to(CategoryColumn::CatId)
                        .into())
                        .filter(CategoryColumn::CatVid.is_in(vids));
                }
            }
            
            // 如果使用 category_ids，添加过滤条件
            if has_category_ids {
                if let Some(category_ids) = &params.category_ids {
                    let ids: Vec<i64> = category_ids
                        .split(",")
                        .filter_map(|x| x.trim().parse().ok())
                        .collect();
                    q = q.filter(NodeCategoriesMapColumn::CatId.is_in(ids));
                }
            }
        }

        // 按标签查询 - 合并处理避免重复连接
        let has_tag_vids = params.tag_vids.as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);
        let has_tag_ids = params.tag_ids.as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_tag_vids || has_tag_ids {
            let mut all_tag_ids: Vec<i64> = Vec::new();
            
            // 如果使用 tag_vids，先通过标签表获取对应的 tag_ids
            if has_tag_vids {
                if let Some(tag_vids) = &params.tag_vids {
                    let vids = tag_vids.split(",").collect::<Vec<&str>>();
                    let tag_ids_from_vids: Vec<i64> = TagEntity::find()
                        .filter(TagColumn::TagVid.is_in(vids))
                        .select_only()
                        .column(TagColumn::TagId)
                        .into_tuple()
                        .all(db)
                        .await?;
                    all_tag_ids.extend(tag_ids_from_vids);
                }
            }
            
            // 如果使用 tag_ids，添加到列表中
            if has_tag_ids {
                if let Some(tag_ids) = &params.tag_ids {
                    let ids: Vec<i64> = tag_ids
                        .split(",")
                        .filter_map(|x| x.trim().parse().ok())
                        .collect();
                    all_tag_ids.extend(ids);
                }
            }
            
            // 去重并连接一次 NodeTagsMapEntity
            if !all_tag_ids.is_empty() {
                all_tag_ids.sort();
                all_tag_ids.dedup();
                q = q.join(JoinType::InnerJoin, NodeEntity::belongs_to(NodeTagsMapEntity)
                    .from(NodeColumn::Nid)
                    .to(NodeTagsMapColumn::Nid)
                    .into())
                    .filter(NodeTagsMapColumn::TagId.is_in(all_tag_ids));
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

        // 收集所有相关的用户ID（创建者与更新者），去重后一次性查询
        let mut user_ids_set: HashSet<i64> = HashSet::new();
        for node in nodes.iter() {
            if node.created_by > 0 {
                user_ids_set.insert(node.created_by);
            }
            if node.updated_by > 0 {
                user_ids_set.insert(node.updated_by);
            }
        }
        let user_ids: Vec<i64> = user_ids_set.into_iter().collect();

        // 批量查询相关用户，构建 uid -> 用户 的映射表
        let user_map: HashMap<i64, UserModel> = if !user_ids.is_empty() {
            let users = UserEntity::find()
                .filter(UserColumn::Uid.is_in(user_ids))
                .all(db)
                .await?;
            users.into_iter().map(|u| (u.uid, u)).collect()
        } else {
            HashMap::new()
        };

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

        // 可选字段
        let mut fields: Vec<String> = Vec::new();

        if let Some(x) = &params.fields {
            fields = x.split(",").map(|x| x.trim().to_string()).collect();
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

            // 从用户映射中取创建者与更新者信息
            let author_user = user_map.get(&node.created_by);
            let updated_user = user_map.get(&node.updated_by);

            let mut node_detail = NodeDetailModel {
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
                author_username: None,
                author_nickname: None,
                updated_by_username: None,
                updated_by_nickname: None,
                categories: node_categories,
                tags: node_tags,
                ..Default::default()
            };

            // 使用 if let 在构建后设置作者与更新者的展示信息，更清晰
            if let Some(u) = author_user {
                node_detail.author_username = Some(u.username.clone());
                node_detail.author_nickname = Some(u.nickname.clone());
            }
            if let Some(u) = updated_user {
                node_detail.updated_by_username = Some(u.username.clone());
                node_detail.updated_by_nickname = Some(u.nickname.clone());
            }

            if fields.contains(&String::from("body")) {
                node_detail.body = Some(node_body.body);
                node_detail.body_format = Some(node_body.body_format);
            }

            result.push(node_detail);
        }

        Ok((result, total))
    }

    /**
     * 获取node内容详情
     * 
     * 当前只实现使用 NodeFilters 中的 nid,vid 参数
     * 
     * 返回详细的node数据：包括categories tags body author 等字段
     */
    pub async fn find_node(
        db: &DatabaseConnection,
        params: &NodeFilters
    ) -> ModelResult<Option<NodeDetailModel>> {
        let (nodes, _) = Self::find_node_list(db, params).await?;

        if nodes.is_empty() {
            return Ok(None);
        }

        let node = nodes.first().unwrap();
        Ok(Some(node.clone()))
    }
}