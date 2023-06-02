use oicnp_core::prelude::sea_orm_migration::prelude::*;
use oicnp_derives::{Column as OicColumn};

// cms_files
#[derive(Iden, OicColumn)]
#[oic(comment = "资源文件表")]
pub enum CmsFiles {
    Table,
    #[oic(type = "string", len = 32, comment = "文件ID")]
    Fid,
    #[oic(type = "string", len = 32, default = "", comment = "用户id")]
    Uid,
    #[oic(type = "string", len = 255, default = "", comment = "文件名称")]
    Filename,
    #[oic(type = "string", len = 255, default = "", comment = "相对路径")]
    Uri,
    #[oic(type = "string", len = 64, default = "", comment = "资源存储位置类型 local,qiniu,oos")]
    Storage,
    #[oic(type = "string", len = 64, default = "")]
    Mime,
    #[oic(type = "smallInteger", len = 1, default = "0")]
    Status,
    #[oic(type = "string", len = 32, default = "", comment = "创建者")]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "string", len = 32, default = "", comment = "更新者")]
    UpdatedBy,
    #[oic(type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}

// cms_user_files_map
#[derive(Iden, OicColumn)]
#[oic(comment = "用户图片表")]
pub enum CmsUserFilesMap {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "用户id")]
    Uid,
    #[oic(type = "string", len = 32, comment = "文件ID")]
    Fid,
    #[oic(type = "string", len = 32, comment = "文件类型")]
    Bundle,
    #[oic(type = "integer", default = 0, comment = "权重")]
    Weight,
    #[oic(type = "string", len = 512, default = "", comment = "替代文本")]
    Alt,
    #[oic(type = "string", len = 512, default = "", comment = "名称")]
    Title,
    #[oic(type = "bigInteger", default = 0, comment = "图片文件宽度")]
    Width,
    #[oic(type = "bigInteger", default = 0, comment = "图片文件高度")]
    Height,
}

// cms_taxonomies
#[derive(Iden, OicColumn)]
#[oic(comment = "分类表")]
pub enum CmsTaxonomies {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "分类id")]
    Tid,
    #[oic(type = "string", len = 255, default = "", comment = "分类可读ID")]
    Vid,
    #[oic(type = "string", len = 32, default = "", comment = "父ID")]
    Pid,
    #[oic(type = "string", len = 128, default = "", comment = "分类名称")]
    Name,
    #[oic(type = "string", len = 512, default = "")]
    Description,
    #[oic(type = "string", len = 20, default = "", comment = "内容类型 html,md,text")]
    DescriptionFormat,
    #[oic(type = "integer", default = 0, comment = "权重")]
    Weight,
}


// cms_tags
#[derive(Iden, OicColumn)]
#[oic(comment = "分类表")]
pub enum CmsTags {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "标签id")]
    TagId,
    #[oic(type = "string", len = 255, default = "", comment = "可读ID")]
    Vid,
    #[oic(type = "string", len = 128, default = "", comment = "分类名称")]
    Name,
    #[oic(type = "integer", default = 0, comment = "权重")]
    Weight,
    #[oic(type = "integer", default = 0, comment = "计数")]
    Count,
}

// cms_comments
#[derive(Iden, OicColumn)]
#[oic(comment = "评论表")]
pub enum CmsComments {
    Table,
    #[oic(type = "string", len = 32, default = "" comment = "评论id")]
    Cid,
    #[oic(type = "string", len = 32, default = "", comment = "用户ID")]
    Uid,
    #[oic(type = "string", len = 32, default = "", comment = "父ID")]
    Pid,
    #[oic(type = "char", len = 1, default = 0)]
    Status,
    #[oic(type = "string", len = 64, default = "", comment = "评论对象类型")]
    Bundle,
    #[oic(type = "string", len = 32, default = "", comment = "评论对象ID")]
    TargetId,
    #[oic(type = "string", len = 512, default = "", comment = "评论主题")]
    Subject,
    #[oic(type = "string", len = 128, default = "", comment = "评论者名称")]
    Name,
    #[oic(type = "string", len = 128, default = "", comment = "评论者邮箱")]
    Email,
    #[oic(type = "string", len = 128, default = "", comment = "评论者主页链接")]
    Homepage,
    #[oic(type = "string", len = 128, default = "", comment = "")]
    Hostname,
    #[oic(type = "string", len = 32, default(""), comment = "")]
    CreatedBy,
    #[oic(type = "string", len = 32, default(""), comment = "")]
    UpdatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// cms_comment_body
#[derive(Iden, OicColumn)]
#[oic(comment = "评论主体表")]
pub enum CmsCommentBody {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "评论id")]
    CommentId,
    #[oic(type = "text", default = "")]
    Body,
    #[oic(type = "string", len = 20, default = "", comment = "评论内容格式")]
    BodyFormat,
}

// cms_nodes
#[derive(Iden, OicColumn)]
#[oic(comment = "节点表")]
pub enum CmsNodes {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "评论id")]
    Nid,
    #[oic(type = "string", len = 255, default = "", comment = "可读ID")]
    Vid,
    #[oic(type = "string", len = 64, default = "", comment = "内容类型 article, page")]
    Bundle,
    #[oic(type = "string", len = 512, default = "")]
    Title,
    #[oic(type = "integer", default = 0, comment = "内容已读计数")]
    Viewed,
    #[oic(type = "char", len = 1, default = "0", comment = "是否删除")]
    Deleted,
    #[oic(type = "datetime", default = "null", comment = "发布时间")]
    PublishedAt,
    #[oic(type = "string", len = 32, default(""))]
    CreatedBy,
    #[oic(type = "string", len = 32, default(""))]
    UpdatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}

// cms_node_body
#[derive(Iden, OicColumn)]
#[oic(comment = "节点内容表")]
pub enum CmsNodeBody {
    Table,
    #[oic(type = "string", len = 32, default = "", comment = "评论id")]
    Nid,
    #[oic(type = "text", default = "", comment = "摘要")]
    Summary,
    #[oic(type = "string", len = 20, default = "", comment = "摘要类型 html,md,text")]
    SummaryFormat,
    #[oic(type = "text", default = "", comment = "主内容")]
    Body,
    #[oic(type = "string", len = 20, default = "", comment = "类型 html,md,text")]
    BodyFormat
}

// cms_node_taxonomies_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点分类关联表")]
pub enum CmsNodeTaxonomiesMap {
    Table,
    #[oic(type = "string", len = 20, default = "", comment = "资源类型")]
    Bundle,
    #[oic(type = "string", len = 32, default = "", comment = "节点id")]
    Nid,
    #[oic(type = "string", len = 32, default = "", comment = "分类id")]
    Tid,
}

// cms_node_tags_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点标签关联表")]
pub enum CmsNodeTagsMap {
    Table,
    #[oic(type = "string", len = 20, default = "", comment = "资源类型")]
    Bundle,
    #[oic(type = "string", len = 32, default = "", comment = "节点id")]
    Nid,
    #[oic(type = "string", len = 32, default = "", comment = "标签id")]
    TagId,
}

// cms_node_files_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点标签关联表")]
pub enum CmsNodeFilesMap {
    Table,
    #[oic(type = "string", len = 20, default = "", comment = "资源类型")]
    Bundle,
    #[oic(type = "string", len = 32, default = "", comment = "节点id")]
    Nid,
    #[oic(type = "string", len = 32, default = "", comment = "文件id")]
    Fid,
    #[oic(type = "string", len = 64, default = "", comment = "用途")]
    Usage,
    #[oic(type = "integer", default = 0, comment = "文件id")]
    Weight,
    #[oic(type = "string", len = 512, default = "", comment = "替代文本")]
    Alt,
    #[oic(type = "string", len = 512, default = "", comment = "名称")]
    Title,
    #[oic(type = "bigInteger", default = 0, comment = "图片文件宽度")]
    Width,
    #[oic(type = "bigInteger", default = 0, comment = "图片文件高度")]
    Height,
}

// cms_node_comments_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点评论关联表")]
pub enum CmsNodeCommentsMap {
    Table,
    #[oic(type = "string", len = 20, default = "", comment = "资源类型")]
    Bundle,
    #[oic(type = "string", len = 32, default = "", comment = "节点id")]
    Nid,
    #[oic(type = "string", len = 32, default = "", comment = "评论id")]
    CommentId,
}

// cms_configs
#[derive(Iden, OicColumn)]
#[oic(comment = "配置表")]
pub enum CmsConfigs {
    Table,
    #[oic(type = "string", len = 64, default = "", comment = "key")]
    Name,
    #[oic(type = "string", len = 512, default = "", comment = "value")]
    Data,
    #[oic(type = "string", len = 32, default = "", comment = "数据类型 json, string, number")]
    DataType,
}
