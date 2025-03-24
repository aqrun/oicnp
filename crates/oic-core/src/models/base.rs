use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

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

#[async_trait]
pub trait ModelCrudHandler<'a> {
    type ReqParams: RequestParamsUpdater + Deserialize<'a> + Serialize + 'a;

    /// 批量创建 node
    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::ReqParams],
    ) -> ModelResult<String>;
}