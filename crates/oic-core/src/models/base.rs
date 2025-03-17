use loco_rs::prelude::ActiveModelTrait;

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