use loco_rs::prelude::*;
use crate::entities::prelude::*;
use sea_orm::prelude::*;

#[async_trait::async_trait]
impl ActiveModelBehavior for UserOnlineActiveModel {
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
}
