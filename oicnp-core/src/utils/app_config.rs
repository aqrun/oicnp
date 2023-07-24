/// 服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    /// 当前服务地址
    pub host: String,
    /// 当前服务端口
    pub port: String,
    /// redis 地址
    pub redis_url: String,
    /// 数据库地址
    pub database_url: String,
    /// graphql 地址
    pub graphql_url: String,
    /// 逻辑删除字段
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    /// 日志目录 “target/logs/"
    pub log_dir: String,
    /// 1000
    pub log_cpu: u64,
    /// "100MB" 日志分割尺寸-单位 KB | MB | GB
    pub log_temp_size: String,
    /// 日志打包格式可选 ”“（空-不压缩） | ”gzip“ | ”zip“ | ”lz4“ (lz4压缩包（非常快）)
    pub log_pack_compress: String,
    /// 日志滚动配置 All：保留全部
    /// KeepTime(Duration)：按时间保留
    /// KeepNum(i64)：按版本保留
    pub log_rolling_type: String,
    /// 日志等级
    pub log_level: String,
    /// 短信缓存队列（mem/redis）
    pub sms_cache_send_key_prefix: String,
    /// jwt 密钥
    pub jwt_secret: String,
    /// 超级管理员用户ID
    pub super_user: Vec<String>,
    /// 白名单接口
    pub white_list_api: Vec<String>,
    /// 权限缓存类型
    pub cache_type: String,
    /// 重试次数
    pub login_fail_retry: u64,
    /// 重试等待时间
    pub login_fail_retry_wait_sec: u64,
}
