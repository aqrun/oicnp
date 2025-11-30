use loco_rs::prelude::*;
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, TransactionTrait, Condition, Order};
use crate::{
    entities::prelude::*,
    utils::{catch_err, utc_now},
    RequestParamsUpdater,
    ModelCrudHandler,
};
use std::time::Duration;
use super::{CreateUserOnlineReqParams, UserOnlineFilters, UpdateUserOnlineReqParams, DeleteUserOnlineReqParams};

#[async_trait::async_trait]
impl ActiveModelBehavior for UserOnlineActiveModel {
}

#[async_trait::async_trait]
impl ModelCrudHandler for UserOnlineModel {
    type DataModel = Self;
    type FilterParams = UserOnlineFilters;
    type CreateReqParams = CreateUserOnlineReqParams;
    type UpdateReqParams = UpdateUserOnlineReqParams;
    type DeleteReqParams = DeleteUserOnlineReqParams;

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        if id < 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let item = UserOnlineEntity::find()
            .filter(UserOnlineColumn::Uid.eq(id))
            .one(db)
            .await?;

        item.ok_or_else(|| {
            ModelError::Any(format!("数据不存在,id: {}", id).into())
        })
    }

    ///
    /// 根据ID查找一个
    /// 
    async fn find_by_vid(_db: &DatabaseConnection, _vid: &str) -> ModelResult<Self> {
        Ok(Self::default())
    }

    ////
    /// 获取note列表
    /// 
    async fn find_list(db: &DatabaseConnection, params: &Self::FilterParams) -> ModelResult<(Vec<Self>, u64)> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        // let mut order = params.get_order();
        let order_by_str = params.get_order_by();

        let order = Order::Desc;

        let mut q = UserOnlineEntity::find();

        if let Some(x) = params.uid {
            if x > 0 {
                q = q.filter(UserOnlineColumn::Uid.eq(x));
            }
        }

        if let Some(x) = &params.username {
            if !x.is_empty() {
                q = q.filter(UserOnlineColumn::Username.contains(x));
            }
        }

        let mut order_by = UserOnlineColumn::LoginAt;

        if order_by_str.eq("login_at") {
            order_by = UserOnlineColumn::LoginAt;
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
        for item in params {
            catch_err(item.validate())?;
        }
        
        let txn = db.begin().await?;
        let mut list: Vec<UserOnlineActiveModel> = Vec::new();

        for item in params.iter() {
            let mut entity = UserOnlineActiveModel {
                ..Default::default()
            };
    
            item.update(&mut entity);
            item.update_by_create(&mut entity);
            list.push(entity);
        }
        
        let _ = UserOnlineEntity::insert_many(list).exec(&txn).await?;
        txn.commit().await?;

        Ok(String::from("批量user_online添加完成"))
    }

    /// 创建 note
    async fn create(db: &DatabaseConnection, params: &Self::CreateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;

        let mut item = UserOnlineActiveModel {
            ..Default::default()
        };

        params.update(&mut item);
        params.update_by_create(&mut item);
    
        let item = item.insert(db).await?;

        Ok(item.uid)
    }

    /// 更新数据
    async fn update(db: &DatabaseConnection, params: &Self::UpdateReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let id = params.uid.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let mut item = Self::find_by_id(db, id)
            .await?
            .into_active_model();

        params.update(&mut item);
    
        let item = item.update(db).await?;

        Ok(item.uid)
    }

    /// 删除数据
    async fn delete_one(db: &DatabaseConnection, params: &Self::DeleteReqParams) -> ModelResult<i64> {
        let id = params.uid.unwrap_or(0);

        if id <= 0 {
            return Err(ModelError::Any(format!("数据不存在,id: {}", id).into()));
        }

        let _res = UserOnlineEntity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(id)
    }
}

impl UserOnlineModel {
    pub async fn upsert(
        db: &DatabaseConnection,
        info: &UserOnlineModel,
    ) -> ModelResult<()> {
        let online_data = UserOnlineEntity::find()
            .filter(UserOnlineColumn::Uid.eq(info.uid))
            .one(db)
            .await?
            .unwrap_or(UserOnlineModel::default());

        // 存在更新数据
        if online_data.uid > 0 {
            let mut item = online_data.into_active_model();
            item.token_id = Set(info.token_id.to_string());
            item.token_expire = Set(info.token_expire);
            item.login_at = Set(info.login_at);
            item.username = Set(info.username.to_string());
            item.dpt_name = Set(info.dpt_name.to_string());
            item.net = Set(info.net.to_string());
            item.ip = Set(info.ip.to_string());
            item.location = Set(info.location.to_string());
            item.device = Set(info.device.to_string());
            item.browser = Set(info.browser.to_string());
            item.os = Set(info.os.to_string());
            let _ = item.update(db).await?;
        } else {
            // 不存在新增
            let item = info.clone().into_active_model();
            let _ = item.insert(db).await?;
        }

        Ok(())
    }


    ///
    /// 数据刷新
    /// 清空全部过期数据
    /// 
    pub async fn refresh(db: &DatabaseConnection) -> ModelResult<()> {
        // 获取7天前的UTC时间
        let seven_days_ago = utc_now() - Duration::from_secs(60 * 60 * 24 * 7);
        let q = UserOnlineEntity::delete_many()
            .filter(
                Condition::all()
                    .add(UserOnlineColumn::LoginAt.lt(seven_days_ago))
                    .add(UserOnlineColumn::LoginAt.is_not_null())
            );

        let _res = q.exec(db).await?;

        Ok(())
    }
}
