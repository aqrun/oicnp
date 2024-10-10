use sea_orm_migration::prelude::*;
use oic_derives::Column as OicColumn;

// cms_files
#[derive(Iden, OicColumn)]
pub enum Files {
    #[oic(comment = "资源文件表")]
    Table,
    #[oic(data_type = "bigInt", len = 20, autoIncrement, primaryKey, comment = "文件ID")]
    FileId,
    #[oic(data_type = "string", len = 32, default = "", comment = "用户id")]
    Uid,
    #[oic(data_type = "string", len = 255, default = "", comment = "文件名称")]
    Filename,
    #[oic(data_type = "string", len = 255, default = "", comment = "相对路径")]
    Uri,
    #[oic(data_type = "string", len = 64, default = "", comment = "资源存储位置类型 local,qiniu,oos")]
    Storage,
    #[oic(data_type = "string", len = 64, default = "")]
    Mime,
    #[oic(data_type = "smallInt", default = "0")]
    Status,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "创建者")]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "更新者")]
    UpdatedBy,
    #[oic(data_type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}

// cms_user_files_map
#[derive(Iden, OicColumn)]
#[oic(comment = "用户图片表")]
pub enum UserFilesMap {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "用户id")]
    Uid,
    #[oic(data_type = "bigInt", len = 20, comment = "文件ID")]
    FileId,
    #[oic(data_type = "string", len = 32, comment = "文件类型")]
    Bundle,
    #[oic(data_type = "int", default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "string", len = 512, default = "", comment = "替代文本")]
    Alt,
    #[oic(data_type = "string", len = 512, default = "", comment = "名称")]
    Title,
    #[oic(data_type = "bigInt", default = 0, comment = "图片文件宽度")]
    Width,
    #[oic(data_type = "bigInt", default = 0, comment = "图片文件高度")]
    Height,
}

// cms_taxonomies
#[derive(Iden, OicColumn)]
#[oic(comment = "分类表")]
pub enum Categories {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "分类id")]
    CatId,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "父ID")]
    CatPid,
    #[oic(data_type = "string", len = 128, default = "", comment = "分类名称")]
    CatName,
    #[oic(data_type = "string", len = 64, default = "", comment = "分类可读ID")]
    CatVid,
    #[oic(data_type = "string", len = 512, default = "")]
    CatDesc,
    #[oic(data_type = "string", len = 20, default = "", comment = "内容类型 html,md,text")]
    CatDescFormat,
    #[oic(data_type = "integer", default = 0, comment = "权重")]
    Weight,
}


// cms_tags
#[derive(Iden, OicColumn)]
#[oic(comment = "分类表")]
pub enum Tags {
    Table,
    #[oic(data_type = "bigInt", comment = "标签id")]
    TagId,
    #[oic(data_type = "string", len = 128, default = "", comment = "可读ID")]
    TagVid,
    #[oic(data_type = "string", len = 128, default = "", comment = "分类名称")]
    TagName,
    #[oic(data_type = "int", default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "int", default = 0, comment = "计数")]
    TagCount,
}

// cms_comments
#[derive(Iden, OicColumn)]
#[oic(comment = "评论表")]
pub enum Comments {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "评论id")]
    CommentId,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "用户ID")]
    Uid,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "父ID")]
    Pid,
    #[oic(data_type = "char", len = 1, default = 0)]
    Status,
    #[oic(data_type = "string", len = 64, default = "", comment = "评论对象类型")]
    Bundle,
    #[oic(data_type = "string", len = 32, default = "", comment = "评论对象ID")]
    TargetId,
    #[oic(data_type = "string", len = 512, default = "", comment = "评论主题")]
    Subject,
    #[oic(data_type = "string", len = 128, default = "", comment = "评论者名称")]
    Name,
    #[oic(data_type = "string", len = 128, default = "", comment = "评论者邮箱")]
    Email,
    #[oic(data_type = "string", len = 128, default = "", comment = "评论者主页链接")]
    Homepage,
    #[oic(data_type = "string", len = 128, default = "", comment = "")]
    Hostname,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    CreatedBy,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    UpdatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// cms_comment_body
#[derive(Iden, OicColumn)]
#[oic(comment = "评论主体表")]
pub enum CommentBody {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "评论id")]
    CommentId,
    #[oic(data_type = "text", default = "")]
    Body,
    #[oic(data_type = "string", len = 20, default = "", comment = "评论内容格式")]
    BodyFormat,
}

// cms_nodes
#[derive(Iden, OicColumn)]
#[oic(comment = "节点表")]
pub enum Nodes {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "内容id")]
    Nid,
    #[oic(data_type = "string", len = 32, comment = "UUID")]
    Uuid,
    #[oic(data_type = "string", len = 255, default = "", comment = "可读ID")]
    Vid,
    #[oic(data_type = "string", len = 64, default = "", comment = "内容类型 article, page")]
    Bundle,
    #[oic(data_type = "string", len = 512, default = "")]
    Title,
    #[oic(data_type = "bigInt", default = 0, comment = "内容已读计数")]
    Viewed,
    #[oic(data_type = "char", len = 1, default = "0", comment = "是否删除")]
    Deleted,
    #[oic(data_type = "datetime", default = "null", comment = "发布时间")]
    PublishedAt,
    #[oic(data_type = "bigInt", len = 20, default = 0)]
    CreatedBy,
    #[oic(data_type = "bigInt", len = 20, default = 0)]
    UpdatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}

// cms_node_body
#[derive(Iden, OicColumn)]
#[oic(comment = "节点内容表")]
pub enum NodeBody {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "评论id")]
    Nid,
    #[oic(data_type = "text", default = "", comment = "摘要")]
    Summary,
    #[oic(data_type = "string", len = 20, default = "", comment = "摘要类型 html,md,text")]
    SummaryFormat,
    #[oic(data_type = "text", default = "", comment = "主内容")]
    Body,
    #[oic(data_type = "string", len = 20, default = "", comment = "类型 html,md,text")]
    BodyFormat
}

// cms_node_taxonomies_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点分类关联表")]
pub enum NodeCategoriesMap {
    Table,
    #[oic(data_type = "string", len = 32, default = "" comment = "资源类型")]
    Bundle,
    #[oic(data_type = "bigInt", len = 20, comment = "节点id")]
    Nid,
    #[oic(data_type = "bigInt", comment = "分类id")]
    CatId,
}

// cms_node_tags_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点标签关联表")]
pub enum NodeTagsMap {
    Table,
    #[oic(data_type = "string", len = 32, default = "", comment = "资源类型")]
    Bundle,
    #[oic(data_type = "bigInt", len = 20, comment = "节点id")]
    Nid,
    #[oic(data_type = "bigInt", comment = "标签id")]
    TagId,
}

// cms_node_files_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点标签关联表")]
pub enum NodeFilesMap {
    Table,
    #[oic(data_type = "string", len = 32, default = "", comment = "资源类型")]
    Bundle,
    #[oic(data_type = "bigInt", len = 20, comment = "节点id")]
    Nid,
    #[oic(data_type = "bigInt", len = 20, comment = "文件id")]
    FileId,
    #[oic(data_type = "string", len = 64, default = "", comment = "用途")]
    Usage,
    #[oic(data_type = "int", default = 0, comment = "文件id")]
    Weight,
    #[oic(data_type = "string", len = 512, default = "", comment = "替代文本")]
    Alt,
    #[oic(data_type = "string", len = 512, default = "", comment = "名称")]
    Title,
    #[oic(data_type = "bigInt", default = 0, comment = "图片文件宽度")]
    Width,
    #[oic(data_type = "bigInt", default = 0, comment = "图片文件高度")]
    Height,
}

// cms_node_comments_map
#[derive(Iden, OicColumn)]
#[oic(comment = "节点评论关联表")]
pub enum NodeCommentsMap {
    Table,
    #[oic(data_type = "string", len = 32, comment = "资源类型")]
    Bundle,
    #[oic(data_type = "bigInt", len = 20, comment = "节点id")]
    Nid,
    #[oic(data_type = "bigInt", len = 20, comment = "评论id")]
    CommentId,
}

// cms_configs
#[derive(Iden, OicColumn)]
#[oic(comment = "配置表")]
pub enum Configs {
    Table,
    #[oic(data_type = "string", len = 64, default = "", comment = "key")]
    Name,
    #[oic(data_type = "string", len = 512, default = "", comment = "value")]
    Data,
    #[oic(data_type = "string", len = 32, default = "", comment = "数据类型 json, string, number")]
    DataType,
}

#[derive(Iden, OicColumn)]
#[oic(comment = "短链接表")]
pub enum ShortLinks {
    Table,
    #[oic(data_type = "bigInt", len = 20)]
    Id,
    #[oic(data_type = "string", len = 32, default "", comment = "key")]
    Vid,
    #[oic(data_type = "string", len = 512, default = "", comment = "名称")]
    Link,
    #[oic(data_type = "string", len = 255, default = "", comment = "名称")]
    Name,
    #[oic(data_type = "string", len = 512, default = "", comment = "简介")]
    Description,
    #[oic(data_type = "bigInt", default = 0, comment = "内容已读计数")]
    Viewed,
    #[oic(data_type = "char", len = 1, default = "0", comment = "是否删除")]
    Deleted,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "bigInt", len = 20, comment = "创建者")]
    CreatedBy,
}
