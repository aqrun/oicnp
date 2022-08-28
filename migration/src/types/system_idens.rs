use sea_orm_migration::prelude::*;
use oicnp_derives::{Column as OicColumn};

#[derive(Iden, OicColumn)]
pub enum SysApiDb {
    Table,
    #[oic()]
    ApiId,
    Db,
}

// sys_dept
#[derive(Iden, OicColumn)]
#[oic(comment = "部门表")]
pub enum SysDepartments {
    Table,
    #[oic(type = "string", len = 32, comment = "部门ID")]
    Id,
    #[oic(type = "string", len = 32, default = "" comment = "父部门id")]
    ParentId,
    #[oic(type = "string", len = 32, default = "" comment = "部门名称")]
    Name,
    #[oic(type = "int", len = 4, default = 0 comment = "权重")]
    Weight,
    #[oic(type = "string", len = 20, default = "" comment = "负责人")]
    Leader,
    #[oic(type = "string", len = 11, default = "" comment = "联系电话")]
    Phone,
    #[oic(type = "string", len = 50, default = "" comment = "邮箱")]
    Email,
    #[oic(type = "char", len = 1, default = "0" comment = "部门状态（0正常 1停用）")]
    Status,
    #[oic(type = "string", len = 32, default = "" comment = "创建者")]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "string", len = 32, default = "" comment = "更新者")]
    UpdatedBy,
    #[oic(type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// sys_dict_type
#[derive(Iden, OicColumn)]
#[oic(comment = "字典类型表")]
pub enum SysAttributes {
    Table,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    Id,
    // 原 type
    #[oic(type = "string", len = 100, unique = true, default = "" comment = "字典类型")]
    Vid,
    #[oic(type = "string", len = 100, default = "" comment = "字典名称")]
    Name,
    #[oic(type = "char", len = 1, default = "0" comment = "状态（0正常 1停用）")]
    Status,
    #[oic(type = "string", len = 500, default = "" comment = "备注")]
    Remark,
    #[oic(type = "string", len = 32, default = "" comment = "创建者")]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "string", len = 32, default = "" comment = "更新者")]
    UpdatedBy,
    #[oic(type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// sys_dict_data
#[derive(Iden, OicColumn)]
#[oic(comment = "字典数据表")]
pub enum SysAttributeValues {
    Table,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    Id,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    AttributeVid,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    Label,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    Value,
    #[oic(type = "int", len = 4, default = 0 comment = "权重")]
    Weight,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    CssClass,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    ListClass,
    #[oic(type = "string", len = 32, default = "" comment = "字典主键")]
    IsDefault,
    Status,
    #[oic(type = "string", len = 500, default = "" comment = "备注")]
    Remark,
    #[oic(type = "string", len = 32, default = "" comment = "创建者")]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "string", len = 32, default = "" comment = "更新者")]
    UpdatedBy,
    #[oic(type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// sys_job
#[derive(Iden)]
pub enum SysCrons {
    Table,
    Id,
    Vid,
    Count,
    RunCount,
    Name,
    Params,
    Group,
    InvokeTarget,
    Expression,
    MisfirePolicy,
    Concurrent,
    Status,
    Remark,
    LastTime,
    NextTime,
    EndTime,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    DeletedAt,
}

// sys_job_log
#[derive(Iden)]
pub enum SysCronLogs {
    Table,
    Id,
    CronId,
    LotId,
    Weight,
    Name,
    Group,
    InvokeTarget,
    Params,
    Message,
    Status,
    ExceptionInfo,
    IsOnce,
    CreatedAt,
    ElapsedTime,
}

// sys_login_log
#[derive(Iden)]
pub enum SysLoginLogs {
    Table,
    Id,
    LoginName,
    Net,
    Ip,
    Location,
    Browser,
    Os,
    Device,
    Status,
    Message,
    LoginAt,
    Module,
}

// sys_menus
#[derive(Iden)]
pub enum SysMenus {
    Table,
    Id,
    Pid,
    Path,
    Name,
    Icon,
    Type,
    Query,
    Weight,
    Api,
    Status,
    Method,
    Component,
    Visible,
    IsCache,
    LogMethod,
    DataCacheMethod,
    IsFrame,
    DataScope,
    Remark,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

// sys_oper_log
#[derive(Iden)]
pub enum SysOperationLogs {
    Table,
    Id,
    TimeId,
    Title,
    BusinessType,
    Method,
    RequestMethod,
    OperatorType,
    Name,
    DepartmentName,
    Url,
    Ip,
    Location,
    Param,
    PathParam,
    JsonResult,
    Status,
    ErrorMessage,
    Duration,
    CreatedAt,
}

// sys_post
#[derive(Iden)]
pub enum SysPositions {
    Table,
    Id,
    Vid,
    Name,
    Weight,
    Status,
    Remark,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
}

// sys_role
#[derive(Iden)]
pub enum SysRoles {
    Table,
    Id,
    Vid,
    Name,
    Weight,
    Scope,
    Status,
    Remark,
    CreatedAt,
    UpdatedAt,
}

// sys_role_api
#[derive(Iden)]
pub enum SysRoleApiMap {
    Table,
    Id,
    RoleId,
    Api,
    Method,
    CreatedBy,
    CreatedAt,
}

// sys_role_dept
#[derive(Iden)]
pub enum SysRoleDepartmentMap {
    Table,
    RoleId,
    DepartmentId,
    CreatedAt,
}

// sys_update_log
#[derive(Iden)]
pub enum SysUpdateLogs {
    Table,
    Id,
    AppVersion,
    BackendVersion,
    Title,
    Content,
    CreatedAt,
    CreatedBy,
    UpdateAt,
    UpdateBy,
    DeletedAt,
}

// sys_user
#[derive(Iden)]
pub enum SysUsers {
    Table,
    Id,
    Username,
    Nickname,
    Password,
    Salt,
    Status,
    Email,
    Gender,
    Avatar,
    RoleId,
    DepartmentId,
    Remark,
    IsAdmin,
    Phone,
    LastLoginIp,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

// sys_user_dept
#[derive(Iden)]
pub enum SysUserDepartmentMap {
    Table,
    UserId,
    DepartmentId,
    CreatedBy,
    CreatedAt,
}

// sys_user_post
#[derive(Iden)]
pub enum SysUserPositionMap {
    Table,
    UserId,
    PositionId,
    CreatedAt,
}

// sys_user_role
#[derive(Iden)]
pub enum SysUserRoleMap {
    Table,
    UserId,
    RoleId,
    CreatedBy,
    CreatedAt,
}

// sys_user_online
#[derive(Iden)]
pub enum SysUserOnline {
    Table,
    UserId,
    TokenId,
    TokenExpire,
    LoginAt,
    Username,
    DepartmentName,
    Net,
    Ip,
    Location,
    Device,
    Browser,
    Os,
}
