use async_trait::async_trait;
use loco_rs::prelude::*;
use loco_rs::model::{ModelError, ModelResult};
use super::{
    PoetryAnalysisView,
    CountDataModel,
    PoetryListDataModel,
    PoetryListPageDataResponse,
};
use crate::utils::catch_err;
use crate::entities::poetry::*;
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder, Condition, Order, QuerySelect, JoinType};
use futures::future::try_join_all;

use super::{
    PoetryFilters,
    CreatePoetryReqParams,
    UpdatePoetryReqParams,
    DeletePoetryReqParams,
};

#[async_trait]
impl ModelCrudHandler for PoetryModel {
    type DataModel = Self;
    type FilterParams = PoetryFilters;
    type CreateReqParams = CreatePoetryReqParams;
    type UpdateReqParams = UpdatePoetryReqParams;
    type DeleteReqParams = DeletePoetryReqParams;

    /// 根据ID查找一个
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self::DataModel> {
        let poetry = PoetryEntity::find()
            .filter(
                model::query::condition()
                    .eq(PoetryColumn::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        poetry.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 根据vid查找一个
    async fn find_by_vid(db: &DatabaseConnection, vid: &str) -> ModelResult<Self::DataModel> {
        let poetry = PoetryEntity::find()
            .filter(
                model::query::condition()
                    .eq(PoetryColumn::Uuid, vid)
                    .build(),
            )
            .one(db)
            .await?;
        poetry.ok_or_else(|| ModelError::EntityNotFound)
    }

    ////
    /// 获取poetry列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = PoetryEntity::find();

        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(PoetryColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Uuid.eq(x));
            }
        }

        if let Some(x) = &params.title {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Title.contains(x));
            }
        }

        if let Some(x) = &params.author_id {
            if *x > 0 {
                q = q.filter(PoetryColumn::AuthorId.eq(*x));
            }
        }

        if let Some(x) = &params.dynasty {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Dynasty.eq(x));
            }
        }

        if let Some(x) = &params.weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::Weight.eq(*x));
            }
        }

        if let Some(x) = &params.hot_weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::HotWeight.eq(*x));
            }
        }

        if let Some(x) = &params.content {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Content.contains(x));
            }
        }

        if let Some(x) = &params.word_count {
            if *x > 0 {
                q = q.filter(PoetryColumn::WordCount.eq(*x));
            }
        }

        if let Some(x) = &params.tags {
            if !x.is_empty() {
                // 支持逗号分隔的多个标签，多个 filter 调用是 AND 关系
                let tag_list: Vec<&str> = x.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                for tag in tag_list {
                    q = q.filter(PoetryColumn::Tags.contains(tag));
                }
            }
        }

        let mut order_by = PoetryColumn::Id;

        if order_by_str.eq("created_at") {
            order_by = PoetryColumn::CreatedAt;
        } else if order_by_str.eq("weight") {
            order_by = PoetryColumn::Weight;
        } else if order_by_str.eq("hot_weight") {
            order_by = PoetryColumn::HotWeight;
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        Ok((list, total))
    }

    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }

        let txn = db.begin().await?;
        let mut poetries: Vec<PoetryActiveModel> = Vec::new();

        for item in params.iter() {
            let mut poetry = PoetryActiveModel::new();
            item.update(&mut poetry);
            item.update_by_create(&mut poetry);

            poetries.push(poetry);
        }

        let _ = PoetryEntity::insert_many(poetries).exec(&txn).await?;

        txn.commit().await?;
        
        Ok(String::from("批量poetry添加完成"))
    }

    /// 创建 poetry
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut poetry = PoetryActiveModel::new();
        params.update(&mut poetry);
        params.update_by_create(&mut poetry);

        let poetry = poetry.insert(db).await?;

        Ok(poetry.id as i64)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.id.unwrap_or(0);

        if id < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", id)));
        }

        let mut poetry = Self::find_by_id(db, id as i64)
            .await?
            .into_active_model();    
        params.update(&mut poetry);
    
        let item = poetry.update(db).await?;

        Ok(item.id as i64)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.id.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Message(format!("数据不存在, id: {}", id)));
        }

        let _res = PoetryEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id as i64)
    }
}

impl PoetryModel {
    /// 根据标题和作者更新诗词搜索排名
    pub async fn update_hot_weight(
        db: &DatabaseConnection,
        params: PoetryFilters,
        hot_weight: i16,
    ) -> ModelResult<i64> {
        let mut cdt = Condition::all();

        if let Some(x) = &params.id {
            if *x > 0 {
                cdt = cdt.add(PoetryColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                cdt = cdt.add(PoetryColumn::Uuid.eq(x));
            }
        }

        if let Some(x) = &params.title {
            if !x.is_empty() {
                cdt = cdt.add(PoetryColumn::Title.like(format!("%{}%", x)));
            }
        }

        if let Some(x) = &params.author_id {
            if *x > 0 {
                cdt = cdt.add(PoetryColumn::AuthorId.eq(*x));
            }
        }

        PoetryEntity::update_many()
            .col_expr(PoetryColumn::HotWeight, Expr::value(hot_weight))
            .filter(cdt)
            .exec(db)
            .await?;

        Ok(0)
    }
    
    /// 获取诗词所有章节
    pub async fn find_all_chapters(db: &DatabaseConnection, poetry_id: i32) -> ModelResult<Vec<ChapterModel>> {
        let chapters = ChapterEntity::find()
            .filter(ChapterColumn::PoetryId.eq(poetry_id))
            .order_by(ChapterColumn::Weight, Order::Asc)
            .all(db)
            .await?;
        Ok(chapters)
    }

    pub async fn upsert(db: &DatabaseConnection, params: &CreatePoetryReqParams) -> ModelResult<(i32, String)> {
        let mut cdt = Condition::all();

        if let Some(x) = &params.title {
            if !x.is_empty() {
                cdt = cdt.add(PoetryColumn::Title.like(format!("%{}%", x)));
            }
        }
        if let Some(x) = &params.author_id {
            if *x > 0 {
                cdt = cdt.add(PoetryColumn::AuthorId.eq(*x));
            }
        }
        if let Some(x) = &params.dynasty {
            if !x.is_empty() {
                cdt = cdt.add(PoetryColumn::Dynasty.eq(x));
            }
        }

        let poetry = PoetryEntity::find()
            .filter(cdt)
            .one(db)
            .await?;

        if let Some(poetry) = poetry {
            let mut poetry = poetry.into_active_model();
            params.update(&mut poetry);
            let p = poetry.update(db).await?;
            let update_or_create = String::from("update");
            return Ok((p.id as i32, update_or_create));
        }
        
        match Self::create(db, params).await {
            Ok(id) => {
                let update_or_create = String::from("create");
                Ok((id as i32, update_or_create))
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn get_analysis_view(db: &DatabaseConnection) -> ModelResult<PoetryAnalysisView> {
        let total_poetry = PoetryEntity::find().count(db).await?;
        let total_author = AuthorEntity::find().count(db).await?;
        
        let total_wen_yan_wen = PoetryEntity::find()
            .filter(PoetryColumn::Tags.like(format!("%{}%", "文言文")))
            .count(db)
            .await?;

        // sum(word_count)
        let res = PoetryEntity::find()
            .select_only()
            .column_as(PoetryColumn::WordCount.sum(), "total_word_count")
            .into_model::<CountDataModel>()
            // .into_tuple::<(u64)>()
            .one(db)
            .await?
            .unwrap_or_default();

        Ok(PoetryAnalysisView {
            total_poetry,
            total_author,
            total_wen_yan_wen,
            total_word_count: res.total_word_count as u64,
        })
    }

    pub async fn get_list_page_data(
        db: &DatabaseConnection,
        params: PoetryFilters,
    ) -> ModelResult<PoetryListPageDataResponse> {
        let poetry_amount = params.poetry_amount.unwrap_or(6);
        let chapter_amount = params.chapter_amount.unwrap_or(3);
        let mut home_categories: Vec<String> = vec![];

        if let Some(tags) = &params.tags {
            home_categories = tags.split(',').map(|s| s.to_string()).collect();
        }
        
        // 为每个分类创建查询任务，并行执行
        let queries: Vec<_> = home_categories
            .into_iter()
            .map(|category| {
                PoetryEntity::find()
                    .select_only()
                    // 选择 poetry 表的所有字段
                    .column(PoetryColumn::Id)
                    .column(PoetryColumn::Uuid)
                    .column(PoetryColumn::Title)
                    .column(PoetryColumn::AuthorId)
                    .column(PoetryColumn::Dynasty)
                    .column(PoetryColumn::Weight)
                    .column(PoetryColumn::HotWeight)
                    .column(PoetryColumn::Content)
                    .column(PoetryColumn::WordCount)
                    .column(PoetryColumn::Tags)
                    .column(PoetryColumn::Description)
                    // .column(PoetryColumn::CreatedAt)
                    // .column(PoetryColumn::UpdatedAt)
                    // 使用 column_as 选择作者信息并设置别名
                    .column_as(AuthorColumn::Uuid, "author_uuid")
                    .column_as(AuthorColumn::Name, "author_name")
                    // Left Join 作者表
                    .join(
                        JoinType::LeftJoin, 
                        PoetryEntity::belongs_to(AuthorEntity)
                        .from(PoetryColumn::AuthorId)
                        .to(AuthorColumn::Id)
                        .into()
                    )
                    .filter(PoetryColumn::Tags.contains(category))
                    .order_by(PoetryColumn::Weight, Order::Asc)
                    .limit(poetry_amount)
                    .into_model::<PoetryListDataModel>()
                    .all(db)
            })
            .collect();
        
        // 并行执行所有查询
        let results = try_join_all(queries).await?;
        
        // 将所有结果合并到一个向量中，同时设置 is_book 字段
        let mut all_poetry: Vec<PoetryListDataModel> = Vec::new();
        let mut book_poetry_ids: Vec<i32> = Vec::new();
        
        // content 是 book 时需要获取全部章节数据 并设置 is_book 为 true
        for poetry_list in results {
            for mut poetry in poetry_list {
                // 在合并时直接判断并设置 is_book 字段
                if poetry.content == "book" {
                    poetry.is_book = Some(String::from("1"));
                    book_poetry_ids.push(poetry.id);
                } else {
                    poetry.is_book = Some(String::from("0"));
                }
                all_poetry.push(poetry);
            }
        }

        // 为每个书籍创建并发查询任务，每个书籍只获取3个章节
        let chapter_queries: Vec<_> = book_poetry_ids
            .iter()
            .map(|&poetry_id| {
                ChapterEntity::find()
                    .filter(ChapterColumn::PoetryId.eq(poetry_id))
                    .order_by(ChapterColumn::Weight, Order::Asc)
                    .limit(chapter_amount)
                    .all(db)
            })
            .collect();

        // 并发执行所有章节查询
        let chapter_results = try_join_all(chapter_queries).await?;

        // 收集所有章节数据
        let mut all_chapters: Vec<ChapterModel> = Vec::new();
        for chapters in chapter_results {
            all_chapters.extend(chapters);
        }

        let res_data = PoetryListPageDataResponse {
            poetry_list: all_poetry,
            chapter_list: all_chapters,
            total: 0,
            page: 0,
            page_size: 0,
        };
        
        Ok(res_data)
    }

    /// 获取poetry列表（带章节数据）
    /// 功能类似 get_list_page_data，但只支持一个分类
    pub async fn find_list_with_chapters(
        db: &DatabaseConnection,
        params: &PoetryFilters,
    ) -> ModelResult<PoetryListPageDataResponse> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();
        let chapter_amount = params.chapter_amount.unwrap_or(3);

        // 构建查询，使用 select_only 和 join 作者表
        let mut q = PoetryEntity::find()
            .select_only()
            .column(PoetryColumn::Id)
            .column(PoetryColumn::Uuid)
            .column(PoetryColumn::Title)
            .column(PoetryColumn::AuthorId)
            .column(PoetryColumn::Dynasty)
            .column(PoetryColumn::Weight)
            .column(PoetryColumn::HotWeight)
            .column(PoetryColumn::Content)
            .column(PoetryColumn::WordCount)
            .column(PoetryColumn::Tags)
            .column(PoetryColumn::Description)
            .column_as(AuthorColumn::Uuid, "author_uuid")
            .column_as(AuthorColumn::Name, "author_name")
            .join(
                JoinType::LeftJoin,
                PoetryEntity::belongs_to(AuthorEntity)
                    .from(PoetryColumn::AuthorId)
                    .to(AuthorColumn::Id)
                    .into()
            );

        // 应用过滤条件
        if let Some(x) = &params.id {
            if *x > 0 {
                q = q.filter(PoetryColumn::Id.eq(*x));
            }
        }

        if let Some(x) = &params.uuid {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Uuid.eq(x));
            }
        }

        if let Some(x) = &params.title {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Title.contains(x));
            }
        }

        if let Some(x) = &params.author_id {
            if *x > 0 {
                q = q.filter(PoetryColumn::AuthorId.eq(*x));
            }
        }

        if let Some(x) = &params.dynasty {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Dynasty.eq(x));
            }
        }

        if let Some(x) = &params.weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::Weight.eq(*x));
            }
        }

        if let Some(x) = &params.hot_weight {
            if *x > 0 {
                q = q.filter(PoetryColumn::HotWeight.eq(*x));
            }
        }

        if let Some(x) = &params.content {
            if !x.is_empty() {
                q = q.filter(PoetryColumn::Content.contains(x));
            }
        }

        if let Some(x) = &params.word_count {
            if *x > 0 {
                q = q.filter(PoetryColumn::WordCount.eq(*x));
            }
        }

        // 只支持一个分类：如果 tags 是逗号分隔的，只取第一个
        if let Some(x) = &params.tags {
            if !x.is_empty() {
                // 只取第一个标签（去除逗号分隔）
                let first_tag = x.split(',').next().unwrap_or(x).trim();
                if !first_tag.is_empty() {
                    q = q.filter(PoetryColumn::Tags.contains(first_tag));
                }
            }
        }

        // 设置排序
        let mut order_by = PoetryColumn::Id;
        if order_by_str.eq("created_at") {
            order_by = PoetryColumn::CreatedAt;
        } else if order_by_str.eq("weight") {
            order_by = PoetryColumn::Weight;
        } else if order_by_str.eq("hot_weight") {
            order_by = PoetryColumn::HotWeight;
        }
        
        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        
        // 分页查询：使用 limit 和 offset，然后使用 into_model 转换
        let poetry_list = q
            .order_by(order_by, order)
            .limit(page_size)
            .offset((page - 1) * page_size)
            .into_model::<PoetryListDataModel>()
            .all(db)
            .await?;

        // 处理 is_book 字段并收集 book 类型的 poetry_id
        let mut all_poetry: Vec<PoetryListDataModel> = Vec::new();
        let mut book_poetry_ids: Vec<i32> = Vec::new();

        for mut poetry in poetry_list {
            if poetry.content == "book" {
                poetry.is_book = Some(String::from("1"));
                book_poetry_ids.push(poetry.id);
            } else {
                poetry.is_book = Some(String::from("0"));
            }
            all_poetry.push(poetry);
        }

        // 为每个书籍创建并发查询任务，获取章节数据
        let chapter_queries: Vec<_> = book_poetry_ids
            .iter()
            .map(|&poetry_id| {
                ChapterEntity::find()
                    .filter(ChapterColumn::PoetryId.eq(poetry_id))
                    .order_by(ChapterColumn::Weight, Order::Asc)
                    .limit(chapter_amount)
                    .all(db)
            })
            .collect();

        // 并发执行所有章节查询
        let chapter_results = try_join_all(chapter_queries).await?;

        // 收集所有章节数据
        let mut all_chapters: Vec<ChapterModel> = Vec::new();
        for chapters in chapter_results {
            all_chapters.extend(chapters);
        }

        let res_data = PoetryListPageDataResponse {
            poetry_list: all_poetry,
            chapter_list: all_chapters,
            total: total as u64,
            page: page as u64,
            page_size: page_size as u64,
        };

        Ok(res_data)
    }
}

impl PoetryActiveModel {
    
}
