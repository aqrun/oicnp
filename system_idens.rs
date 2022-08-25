use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum SysApiDb {
    Table,
    ApiId,
    Db,
}

// sys_dept
#[derive(Iden)]
pub enum SysDepartments {
    Table,
    Id,
    ParentId,
    Name,
    Weight,
    Leader,
    Phone,
    Email,
    Status,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    DeletedAt,
}

// sys_dict_type
#[derive(Iden)]
pub enum SysAttributes {
    Table,
    Id,
    // 原 type
    Vid,
    Name,
    Status,
    Remark,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    DeletedAt,
}

// sys_dict_data
#[derive(Iden)]
pub enum SysAttributeValues {
    Table,
    Id,
    AttributeVid,
    Label,
    Value,
    Weight,
    CssClass,
    ListCalss,
    IsDefault,
    Status,
    Remark,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
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
    BussinessType,
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
    UIpdatedAt,
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
pub enum SysRoleDeptartmentMap {
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
