use sea_orm_migration::prelude::*;
use oic_derives::Column as OicColumn;

#[derive(Iden, OicColumn)]
pub enum ApiDb {
    Table,
    // #[oic()]
    ApiId,
    Db,
}

// sys_dept
#[derive(Iden, OicColumn)]
#[oic(comment = "部门表")]
pub enum Departments {
    Table,
    #[oic(data_type = "bigInt", comment = "部门ID")]
    DptId,
    #[oic(data_type = "bigInt", default = 0, comment = "父部门id")]
    Pid,
    #[oic(data_type = "string", len = 32, default = "", comment = "部门名称")]
    Name,
    #[oic(data_type = "int", len = 4, default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "string", len = 20, default = "", comment = "负责人")]
    Leader,
    #[oic(data_type = "string", len = 11, default = "", comment = "联系电话")]
    Phone,
    #[oic(data_type = "string", len = 50, default = "", comment = "邮箱")]
    Email,
    #[oic(data_type = "char", len = 1, default = "0", comment = "部门状态（0正常 1停用）")]
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

// sys_dict_type
#[derive(Iden, OicColumn)]
#[oic(comment = "字典类型表")]
pub enum Attributes {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "字典主键")]
    Id,
    // 原 type
    #[oic(data_type = "string", len = 100, unique = true, default = "", comment = "字典类型")]
    Vid,
    #[oic(data_type = "string", len = 100, default = "", comment = "字典名称")]
    Name,
    #[oic(data_type = "char", len = 1, default = "0", comment = "状态（0正常 1停用）")]
    Status,
    #[oic(data_type = "string", len = 500, default = "", comment = "备注")]
    Remark,
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

// sys_dict_data
#[derive(Iden, OicColumn)]
#[oic(comment = "字典数据表")]
pub enum AttributeValues {
    Table,
    #[oic(data_type = "bigInt", len = 20, default = "", comment = "字典主键")]
    Id,
    #[oic(data_type = "string", len = 100, default = "", comment = "字典类型")]
    Vid,
    #[oic(data_type = "string", len = 100, default = "", comment = "字典标签")]
    Label,
    #[oic(data_type = "string", len = 100, default = "", comment = "字典键值")]
    Value,
    #[oic(data_type = "int", len = 4, default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "string", len = 100, default = "", comment = "样式属性（其他样式扩展）")]
    CssClass,
    #[oic(data_type = "string", len = 100, default = "", comment = "表格回显样式")]
    ListClass,
    #[oic(data_type = "char", len = 1, default = "N", comment = "是否默认（Y是 N否）")]
    IsDefault,
    #[oic(data_type = "char", len = 1, default = "0", comment = "状态（0正常 1停用）")]
    Status,
    #[oic(data_type = "string", len = 500, default = "", comment = "备注")]
    Remark,
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

// sys_job
#[derive(Iden, OicColumn)]
#[oic(comment = "定时任务调度表")]
pub enum Crons {
    Table,
    #[oic(data_type = "bigInt", comment = "任务ID")]
    Id,
    #[oic(data_type = "string", len = 100, default = "", comment = "vid")]
    Vid,
    #[oic(data_type = "int", default = 0, comment = "")]
    Count,
    #[oic(data_type = "int", default = 0)]
    RunCount,
    #[oic(data_type = "string", len = 64, default = "", comment = "任务名称")]
    Name,
    #[oic(data_type = "string", len = 200, default = "", comment = "任务参数")]
    Params,
    #[oic(data_type = "string", len = 64, default = "DEFAULT", comment = "任务组名")]
    Group,
    #[oic(data_type = "string", len = 500, default = "", comment = "调用目标字符串")]
    InvokeTarget,
    #[oic(data_type = "string", len = 255, default = "", comment = "cron执行表达式")]
    Expression,
    #[oic(data_type = "string", len = 20, default = "3", comment = "计划执行错误策略（1立即执行 2执行一次 3放弃执行）")]
    MisfirePolicy,
    #[oic(data_type = "char", len = 1, default = "1", comment = "是否并发执行（0允许 1禁止）")]
    Concurrent,
    #[oic(data_type = "char", len = 1, default = "1", comment = "状态（1正常 0暂停）")]
    Status,
    #[oic(data_type = "string", len = 500, default = "", comment = "备注信息")]
    Remark,
    #[oic(data_type = "datetime", default = "null", comment = "上次执行时间")]
    LastTime,
    #[oic(data_type = "datetime", default = "null", comment = "下次执行时间")]
    NextTime,
    #[oic(data_type = "datetime", default = "null", comment = "结束时间")]
    EndTime,
    #[oic(data_type = "bigInt", len = 20, default = 0 comment = "创建者")]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "bigInt", len = 20, default = 0 comment = "更新者")]
    UpdatedBy,
    #[oic(data_type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}

// sys_job_log
#[derive(Iden, OicColumn)]
pub enum CronLogs {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "任务日志ID")]
    Id,
    #[oic(data_type = "bigInt", len = 20, comment = "cron id")]
    CronId,
    #[oic(data_type = "bitInt", len = 20, default = 0, comment = "")]
    LotId,
    #[oic(data_type = "int", len = 4, default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "string", len = 64, default = "", comment = "任务名称")]
    Name,
    #[oic(data_type = "string", len = 64, default = "", comment = "任务组名")]
    Group,
    #[oic(data_type = "string", len = 500, default = "", comment = "调用目标字符串")]
    InvokeTarget,
    #[oic(data_type = "string", len = 500, default = "", comment = "参数")]
    Params,
    #[oic(data_type = "string", len = 500, default = "", comment = "日志信息")]
    Message,
    #[oic(data_type = "char", len = 1, default = "1", comment = "状态（1正常 0暂停）")]
    Status,
    #[oic(data_type = "string", len = 2000, default = "", comment = "异常信息")]
    ExceptionInfo,
    #[oic(data_type = "char", default = "null", comment = "")]
    IsOnce,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", comment = "")]
    ElapsedTime,
}

// sys_login_log
#[derive(Iden, OicColumn)]
pub enum LoginLogs {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "id")]
    Id,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    LoginName,
    #[oic(data_type = "string", len = 10, default = "", comment = "")]
    Net,
    #[oic(data_type = "string", len = 50, default = "", comment = "IP")]
    Ip,
    #[oic(data_type = "string", len = 255, default = "", comment = "地址")]
    Location,
    #[oic(data_type = "string", len = 50, default = "", comment = "浏览器")]
    Browser,
    #[oic(data_type = "string", len = 50, default = "", comment = "系统")]
    Os,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    Device,
    #[oic(data_type = "char", len = 1, default = "0", comment = "")]
    Status,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Message,
    #[oic(data_type = "datetime", comment = "")]
    LoginAt,
    #[oic(data_type = "string", len = 30, default = "", comment = "")]
    Module,
}

// sys_menus
#[derive(Iden, OicColumn)]
pub enum Menus {
    Table,
    #[oic(data_type = "bigInt", comment = "id")]
    Id,
    #[oic(data_type = "string", default = "", comment = "")]
    Mid,
    #[oic(data_type = "string", default = "", comment = "")]
    Pid,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Path,
    #[oic(data_type = "string", len = 100, default = "", comment = "")]
    Name,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    Icon,
    #[oic(data_type = "int", default = 0, comment = "")]
    Weight,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Api,
    #[oic(data_type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(data_type = "char", len = 1, default = "1", comment = "")]
    Visible,
    #[oic(data_type = "char", len = 1, default = "1", comment = "")]
    IsCache,
    #[oic(data_type = "char", len = 1, default = "0", comment = "")]
    IsFrame,
    #[oic(data_type = "int", default = 0, comment = "")]
    Depth,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P1,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P2,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P3,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P4,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P5,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P6,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P7,
    #[oic(data_type = "bigInt", default = 0, comment = "")]
    P8,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Remark,
    #[oic(data_type = "datetime", default = "", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null", comment = "删除时间")]
    DeletedAt,
}


// sys_oper_log
#[derive(Iden, OicColumn)]
pub enum OperationLogs {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "ID")]
    Id,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    TimeId,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    Title,
    #[oic(data_type = "char", len = 100, default = "", comment = "")]
    BusinessType,
    #[oic(data_type = "string", len = 100, default = "", comment = "")]
    Method,
    #[oic(data_type = "string", len = 10, default = "", comment = "")]
    RequestMethod,
    #[oic(data_type = "char", len = 100, default = "1", comment = "")]
    OperatorType,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    Name,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    DepartmentName,
    #[oic(data_type = "string", len = 5000, default = "", comment = "")]
    Url,
    #[oic(data_type = "string", len = 50, default = "", comment = "")]
    Ip,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Location,
    #[oic(data_type = "text", default = "", comment = "")]
    Param,
    #[oic(data_type = "text", default = "", comment = "")]
    PathParam,
    #[oic(data_type = "text", default = "", comment = "")]
    JsonResult,
    #[oic(data_type = "char", len = 100, default = "1", comment = "")]
    Status,
    #[oic(data_type = "string", len = 2000, default = "", comment = "")]
    ErrorMessage,
    #[oic(data_type = "bigint", default = 0, comment = "")]
    Duration,
    #[oic(data_type = "datetime", comment = "")]
    CreatedAt,
}

// sys_post
#[derive(Iden, OicColumn)]
pub enum Positions {
    Table,
    #[oic(data_type = "bigInt", comment = "")]
    PositionId,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Vid,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Name,
    #[oic(data_type = "int", default = 0, comment = "权重")]
    Weight,
    #[oic(data_type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(data_type = "string", len = 500, default = "", comment = "")]
    Remark,
    #[oic(data_type = "bigInt", len = 20, default = 0 comment = "创建者")]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "bigInt", len = 20, default = 0 comment = "更新者")]
    UpdatedBy,
    #[oic(data_type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// sys_role
#[derive(Iden, OicColumn)]
pub enum Roles {
    Table,
    #[oic(data_type = "bigInt", comment = "")]
    RoleId,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Vid,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Name,
    #[oic(data_type = "int", default = 0, comment = "")]
    Weight,
    #[oic(data_type = "char", len = 1, default = "0", comment = "")]
    Scope,
    #[oic(data_type = "char", len = 1, default = "1", comment = "")]
    Status,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Remark,
    #[oic(data_type = "datetime", comment = "")]
    CreatedAt,
    #[oic(data_type = "datetime", comment = "")]
    UpdatedAt,
}

// sys_role_api
#[derive(Iden, OicColumn)]
pub enum RoleApiMap {
    Table,
    #[oic(data_type = "bigInt", comment = "")]
    Id,
    #[oic(data_type = "bigInt", comment = "")]
    RoleId,
    #[oic(data_type = "string", len = 255, comment = "")]
    Api,
    #[oic(data_type = "char", len = 10, default = "", comment = "")]
    Method,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "")]
    CreatedAt,
}

// sys_role_dept
#[derive(Iden, OicColumn)]
// #[oic_index(columns = ["role_id", "department_id"])]
pub enum RoleDepartmentMap {
    Table,
    #[oic(data_type = "bigInt", comment = "")]
    RoleId,
    #[oic(data_type = "bigInt", comment = "")]
    DptId,
    #[oic(data_type = "datetime", comment = "")]
    CreatedAt,
}

// sys_update_log
#[derive(Iden, OicColumn)]
pub enum UpdateLogs {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "")]
    Id,
    #[oic(data_type = "char", len = 10, comment = "")]
    AppVersion,
    #[oic(data_type = "char", len = 10, default = "", comment = "")]
    BackendVersion,
    #[oic(data_type = "string", len = 100, default = "", comment = "")]
    Title,
    #[oic(data_type = "text", default = "", comment = "")]
    Content,
    #[oic(data_type = "bigInt", len = 20, default = 0 comment = "创建者")]
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

// sys_users
#[derive(Iden, OicColumn)]
pub enum Users {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "用户ID")]
    Uid,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Uuid,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Username,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Nickname,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Password,
    #[oic(data_type = "string", len = 64, default = "", comment = "")]
    Salt,
    #[oic(data_type = "string", len = 128, default = "", comment = "接口KEY")]
    ApiKey,
    #[oic(data_type = "string", len = 128, default = "", comment = "密码重置凭证")]
    ResetToken,
    #[oic(data_type = "datetime", default = "null", comment = "凭证生成时间")]
    ResetSentAt,
    #[oic(data_type = "string", len = 128, default = "", comment = "密码重置凭证")]
    EmailVerifyToken,
    #[oic(data_type = "datetime", default = "null", comment = "凭证生成时间")]
    EmailVerifySentAt,
    #[oic(data_type = "datetime", default = "null", comment = "凭证生成时间")]
    EmailVerifiedAt,
    #[oic(data_type = "char", len = 1, default = "", comment = "")]
    Status,
    #[oic(data_type = "string", len = 100, default = "", comment = "")]
    Email,
    #[oic(data_type = "char", len = 1, default = "0", comment = "")]
    Gender,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Avatar,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    RoleId,
    #[oic(data_type = "bigInt", len = 20, default = 0, comment = "")]
    DptId,
    #[oic(data_type = "string", len = 255, default = "", comment = "")]
    Remark,
    #[oic(data_type = "char", len = 1, default = "0", comment = "")]
    IsAdmin,
    #[oic(data_type = "string", len = 20, default = "", comment = "")]
    Phone,
    #[oic(data_type = "string", len = 20, default = "", comment = "")]
    LastLoginIp,
    #[oic(data_type = "datetime", default = "null", comment = "")]
    LastLoginAt,
    #[oic(data_type = "bigInt", len = 20, default = "", comment = "")]
    CreatedBy,
    #[oic(data_type = "bigInt", len = 20, default = "", comment = "")]
    UpdatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
    #[oic(data_type = "datetime", default = "null" comment = "更新时间")]
    UpdatedAt,
    #[oic(data_type = "datetime", default = "null" comment = "删除时间")]
    DeletedAt,
}

// sys_user_dept
#[derive(Iden, OicColumn)]
pub enum UserDepartmentMap {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "")]
    Uid,
    #[oic(data_type = "bigInt", comment = "")]
    DptId,
    #[oic(data_type = "string", len = 32)]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_post
#[derive(Iden, OicColumn)]
pub enum UserPositionMap {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "")]
    Uid,
    #[oic(data_type = "bigInt", comment = "")]
    PositionId,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_role
#[derive(Iden, OicColumn)]
pub enum UserRoleMap {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "")]
    Uid,
    #[oic(data_type = "bigInt", comment = "")]
    RoleId,
    #[oic(data_type = "string", len = 32)]
    CreatedBy,
    #[oic(data_type = "datetime", comment = "创建时间")]
    CreatedAt,
}

// sys_user_online
#[derive(Iden, OicColumn)]
pub enum UserOnline {
    Table,
    #[oic(data_type = "bigInt", len = 20, comment = "")]
    Uid,
    #[oic(data_type = "string", len = 32, comment = "")]
    TokenId,
    #[oic(data_type = "bigInt", comment = "")]
    TokenExpire,
    #[oic(data_type = "datetime", comment = "登录时间")]
    LoginAt,
    #[oic(data_type = "string", len = 60, comment = "")]
    Username,
    #[oic(data_type = "string", len = 100, comment = "")]
    DptName,
    #[oic(data_type = "string", len = 10, comment = "")]
    Net,
    #[oic(data_type = "string", len = 100, comment = "")]
    Ip,
    #[oic(data_type = "string", len = 255, comment = "")]
    Location,
    #[oic(data_type = "string", len = 50, comment = "")]
    Device,
    #[oic(data_type = "string", len = 30, comment = "")]
    Browser,
    #[oic(data_type = "string", len = 30, comment = "")]
    Os,
}

