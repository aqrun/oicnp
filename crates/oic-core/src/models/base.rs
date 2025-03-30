use loco_rs::prelude::*;
use serde::Deserialize;

///
/// 请求参数更新操作
/// 
pub trait RequestParamsUpdater {
    type ActiveModel: ActiveModelTrait;

    ///
    /// 正常数据更新
    /// 
    fn update(&self, item: &mut Self::ActiveModel);

    ///
    /// 新增时相关数据设置
    /// 
    fn update_by_create(&self, item: &mut Self::ActiveModel);
}

/// 模型通用Trait实现 增删改查相关操作
#[async_trait]
pub trait ModelCrudHandler {
    /// 关联的请求参数类型
    type CreateReqParams: RequestParamsUpdater + Deserialize<'static> + Default + Clone + Send;

    /// 批量创建
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String>;
}