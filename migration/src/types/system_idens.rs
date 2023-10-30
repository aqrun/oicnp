use oicnp_core::prelude::sea_orm_migration::prelude::*;
use oicnp_derives::Column as OicColumn;

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
    #[oic(type = "string", len = 100, default = "" comment = "字典类型")]
    AttributeVid,
    #[oic(type = "string", len = 100, default = "" comment = "字典标签")]
    Label,
    #[oic(type = "string", len = 100, default = "" comment = "字典键值")]
    Value,
    #[oic(type = "int", len = 4, default = 0 comment = "权重")]
    Weight,
    #[oic(type = "string", len = 100, default = "" comment = "样式属性（其他样式扩展）")]
    CssClass,
    #[oic(type = "string", len = 100, default = "" comment = "表格回显样式")]
    ListClass,
    #[oic(type = "char", len = 1, default = "N" comment = "是否默认（Y是 N否）")]
    IsDefault,
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

// sys_job
#[derive(Iden, OicColumn)]
#[oic(comment = "定时任务调度表")]
pub enum SysCrons {
    Table,
    #[oic(type = "string", len = 32, comment = "任务ID")]
    Id,
    #[oic(type = "string", len = 100, default = "", comment = "vid")]
    Vid,
    #[oic(type = "int", default = 0, comment = "")]
    Count,
    #[oic(type = "int", default = 0)]
    RunCount,
    #[oic(type = "string", len = 64, default = "", comment = "任务名称")]
    Name,
    #[oic(type = "string", len = 200, default = "", comment = "任务参数")]
    Params,
    #[oic(type = "string", len = 64, default = "DEFAULT", comment = "任务组名")]
    Group,
    #[oic(type = "string", len = 500, default = "", comment = "调用目标字符串")]
    InvokeTarget,
    #[oic(type = "string", len = 255, default = "", comment = "cron执行表达式")]
    Expression,
    #[oic(type = "string", len = 20, default = "3", comment = "计划执行错误策略（1立即执行 2执行一次 3放弃执行）")]
    MisfirePolicy,
    #[oic(type = "char", len = 1, default = "1", comment = "是否并发执行（0允许 1禁止）")]
    Concurrent,
    #[oic(type = "char", len = 1, default = "1", comment = "状态（1正常 0暂停）")]
    Status,
    #[oic(type = "string", len = 500, default = "", comment = "备注信息")]
    Remark,
    #[oic(type = "datetime", default = "null", comment = "上次执行时间")]
    LastTime,
    #[oic(type = "datetime", default = "null", comment = "下次执行时间")]
    NextTime,
    #[oic(type = "datetime", default = "null", comment = "结束时间")]
    EndTime,
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

// sys_job_log
#[derive(Iden, OicColumn)]
pub enum SysCronLogs {
    Table,
    #[oic(type = "string", len = 32, comment = "任务日志ID")]
    Id,
    #[oic(type = "string", len = 32, comment = "cron id")]
    CronId,
    #[oic(type = "string", len = 32, default = "", comment = "")]
    LotId,
    #[oic(type = "int", len = 4, default = 0, comment = "权重")]
    Weight,
    #[oic(type = "string", len = 64, default = "", comment = "任务名称")]
    Name,
    #[oic(type = "string", len = 64, default = "", comment = "任务组名")]
    Group,
    #[oic(type = "string", len = 500, default = "", comment = "调用目标字符串")]
    InvokeTarget,
    #[oic(type = "string", len = 500, default = "", comment = "参数")]
    Params,
    #[oic(type = "string", len = 500, default = "", comment = "日志信息")]
    Message,
    #[oic(type = "char", len = 1, default = "1", comment = "状态（1正常 0暂停）")]
    Status,
    #[oic(type = "string", len = 2000, default = "", comment = "异常信息")]
    ExceptionInfo,
    #[oic(type = "char", default = "null", comment = "")]
    IsOnce,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "datetime", comment = "")]
    ElapsedTime,
}

// sys_login_log
#[derive(Iden, OicColumn)]
pub enum SysLoginLogs {
    Table,
    #[oic(type = "string", len = 32, comment = "id")]
    Id,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    LoginName,
    #[oic(type = "string", len = 10, default = "", comment = "")]
    Net,
    #[oic(type = "string", len = 50, default = "", comment = "IP")]
    Ip,
    #[oic(type = "string", len = 255, default = "", comment = "地址")]
    Location,
    #[oic(type = "string", len = 50, default = "", comment = "浏览器")]
    Browser,
    #[oic(type = "string", len = 50, default = "", comment = "系统")]
    Os,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Device,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    Status,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Message,
    #[oic(type = "datetime", comment = "")]
    LoginAt,
    #[oic(type = "string", len = 30, default = "", comment = "")]
    Module,
}

// sys_menus
#[derive(Iden, OicColumn)]
pub enum SysMenus {
    Table,
    #[oic(type = "string", len = 32, comment = "id")]
    Id,
    #[oic(type = "string", len = 32, default = "", comment = "")]
    Pid,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Path,
    #[oic(type = "string", len = 100, default = "", comment = "")]
    Name,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Icon,
    #[oic(type = "char", len = 1, default = "", comment = "")]
    Type,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Query,
    #[oic(type = "int", default = 0, comment = "")]
    Weight,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Api,
    #[oic(type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(type = "string", len = 10, default = "", comment = "")]
    Method,
    #[oic(type = "string", len = 100, default = "", comment = "")]
    Component,
    #[oic(type = "char", len = 1, default = "1", comment = "")]
    Visible,
    #[oic(type = "char", len = 1, default = "1", comment = "")]
    IsCache,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    LogMethod,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    DataCacheMethod,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    IsFrame,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    DataScope,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Remark,
    #[oic(type = "datetime", default = "", comment = "创建时间")]
    CreatedAt,
    #[oic(type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}


// sys_oper_log
#[derive(Iden, OicColumn)]
pub enum SysOperationLogs {
    Table,
    #[oic(type = "string", len = 32, comment = "ID")]
    Id,
    #[oic(type = "bigint", default = 0, comment = "")]
    TimeId,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Title,
    #[oic(type = "char", len = 100, default = "", comment = "")]
    BusinessType,
    #[oic(type = "string", len = 100, default = "", comment = "")]
    Method,
    #[oic(type = "string", len = 10, default = "", comment = "")]
    RequestMethod,
    #[oic(type = "char", len = 100, default = "1", comment = "")]
    OperatorType,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Name,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    DepartmentName,
    #[oic(type = "string", len = 5000, default = "", comment = "")]
    Url,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Ip,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Location,
    #[oic(type = "text", default = "", comment = "")]
    Param,
    #[oic(type = "text", default = "", comment = "")]
    PathParam,
    #[oic(type = "text", default = "", comment = "")]
    JsonResult,
    #[oic(type = "char", len = 100, default = "1", comment = "")]
    Status,
    #[oic(type = "string", len = 2000, default = "", comment = "")]
    ErrorMessage,
    #[oic(type = "bigint", default = 0, comment = "")]
    Duration,
    #[oic(type = "datetime", comment = "")]
    CreatedAt,
}

// sys_post
#[derive(Iden, OicColumn)]
pub enum SysPositions {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Id,
    #[oic(type = "string", len = 64, default = "", comment = "")]
    Vid,
    #[oic(type = "string", len = 50, default = "", comment = "")]
    Name,
    #[oic(type = "int", default = 0, comment = "权重")]
    Weight,
    #[oic(type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(type = "string", len = 500, default = "", comment = "")]
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

// sys_role
#[derive(Iden, OicColumn)]
pub enum SysRoles {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Id,
    #[oic(type = "string", len = 64, default = "", comment = "")]
    Vid,
    #[oic(type = "string", len = 64, default = "", comment = "")]
    Name,
    #[oic(type = "int", default = 0, comment = "")]
    Weight,
    #[oic(type = "char", len = 1, default = "0", comment = "")]
    Scope,
    #[oic(type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(type = "string", len = 255, default = "", comment = "")]
    Remark,
    #[oic(type = "datetime", comment = "")]
    CreatedAt,
    #[oic(type = "datetime", comment = "")]
    UpdatedAt,
}

// sys_role_api
#[derive(Iden, OicColumn)]
pub enum SysRoleApiMap {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Id,
    #[oic(type = "string", len = 32, comment = "")]
    RoleId,
    #[oic(type = "string", len = 255, comment = "")]
    Api,
    #[oic(type = "char", len = 10, default = "", comment = "")]
    Method,
    #[oic(type = "datetime", comment = "")]
    CreatedBy,
    #[oic(type = "datetime", comment = "")]
    CreatedAt,
}

// sys_role_dept
#[derive(Iden, OicColumn)]
// #[oic_index(columns = ["role_id", "department_id"])]
pub enum SysRoleDepartmentMap {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    RoleId,
    #[oic(type = "string", len = 32, comment = "")]
    DepartmentId,
    #[oic(type = "datetime", comment = "")]
    CreatedAt,
}

// sys_update_log
#[derive(Iden, OicColumn)]
pub enum SysUpdateLogs {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Id,
    #[oic(type = "char", len = 10, comment = "")]
    AppVersion,
    #[oic(type = "char", len = 10, default(""), comment = "")]
    BackendVersion,
    #[oic(type = "string", len = 100, default(""), comment = "")]
    Title,
    #[oic(type = "text", default(""), comment = "")]
    Content,
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

// sys_users
#[derive(Iden, OicColumn)]
pub enum SysUsers {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Uid,
    #[oic(type = "string", len = 60, default(""), comment = "")]
    Username,
    #[oic(type = "string", len = 60, default(""), comment = "")]
    Nickname,
    #[oic(type = "string", len = 64, default(""), comment = "")]
    Password,
    #[oic(type = "string", len = 64, default(""), comment = "")]
    Salt,
    #[oic(type = "char", len = 1, default(""), comment = "")]
    Status,
    #[oic(type = "string", len = 100, default("1"), comment = "")]
    Email,
    #[oic(type = "char", len = 1, default("0"), comment = "")]
    Gender,
    #[oic(type = "string", len = 255, default(""), comment = "")]
    Avatar,
    #[oic(type = "string", len = 32, default(""), comment = "")]
    RoleId,
    #[oic(type = "string", len = 32, default(""), comment = "")]
    DepartmentId,
    #[oic(type = "string", len = 255, default(""), comment = "")]
    Remark,
    #[oic(type = "char", len = 1, default("0"), comment = "")]
    IsAdmin,
    #[oic(type = "string", len = 20, default(""), comment = "")]
    Phone,
    #[oic(type = "string", len = 20, default(""), comment = "")]
    LastLoginIp,
    #[oic(type = "datetime", default("null"), comment = "")]
    LastLoginAt,
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

// sys_user_dept
#[derive(Iden, OicColumn)]
pub enum SysUserDepartmentMap {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Uid,
    #[oic(type = "string", len = 32, comment = "")]
    DepartmentId,
    #[oic(type = "string", len = 32)]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_post
#[derive(Iden, OicColumn)]
pub enum SysUserPositionMap {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Uid,
    #[oic(type = "string", len = 32, comment = "")]
    PositionId,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_role
#[derive(Iden, OicColumn)]
pub enum SysUserRoleMap {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Uid,
    #[oic(type = "string", len = 32, comment = "")]
    RoleId,
    #[oic(type = "string", len = 32)]
    CreatedBy,
    #[oic(type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_online
#[derive(Iden, OicColumn)]
pub enum SysUserOnline {
    Table,
    #[oic(type = "string", len = 32, comment = "")]
    Uid,
    #[oic(type = "string", len = 32, comment = "")]
    TokenId,
    #[oic(type = "biginteger", comment = "")]
    TokenExpire,
    #[oic(type = "datetime", comment = "登录时间")]
    LoginAt,
    #[oic(type = "string", len = 60, comment = "")]
    Username,
    #[oic(type = "string", len = 100, comment = "")]
    DepartmentName,
    #[oic(type = "string", len = 10, comment = "")]
    Net,
    #[oic(type = "string", len = 100, comment = "")]
    Ip,
    #[oic(type = "string", len = 255, comment = "")]
    Location,
    #[oic(type = "string", len = 50, comment = "")]
    Device,
    #[oic(type = "string", len = 30, comment = "")]
    Browser,
    #[oic(type = "string", len = 30, comment = "")]
    Os,
}

