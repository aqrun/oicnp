# 异常 IP 分析脚本使用说明

## 功能

`ips.js` 脚本用于分析 Nginx 错误日志，自动识别异常 IP 地址。

### 支持的文件格式

脚本支持以下日志文件格式：

- `.log` 文件：如 `error.log`, `access.log`
- `.log-YYYYMMDD` 格式：如 `admin_lxage_error.log-20251228`
- `.gz` 压缩文件：如 `admin_lxage_error.log-20251219.gz`
- `.log-YYYYMMDD.gz` 格式：如 `api_lxage_access.log-20251220.gz`

脚本会自动识别并处理这些文件，包括自动解压 `.gz` 压缩文件。

## 异常判定规则

### 1. 特定路径匹配

脚本会检查以下类型的请求路径：

- **配置文件**: `/package.json`, `/config`, `/.env`, `/settings.py` 等
- **版本控制**: `/.git`, `/.github`, `/.gitlab` 等
- **WordPress**: `/wp-admin`, `/wp-content`, `/wp-login.php` 等
- **管理后台**: `/admin`, `/admin.php`, `/administrator` 等
- **敏感文件**: `/.htaccess`, `/web.config`, `/.well-known` 等
- **PHP 漏洞扫描**: `/phpinfo.php`, `/shell.php`, `/1.php` 等

### 2. 模式匹配

使用正则表达式匹配以下模式：

- `.php` 文件
- `.env` 相关文件
- `wp-` 开头的路径（WordPress）
- `admin` 相关路径
- `config` 相关路径
- `secret`, `key`, `credential`, `password` 等敏感词
- `.well-known` 路径
- `autoload` 相关
- Java 打包文件（`.jar`, `.war`, `.ear`）
- 其他常见攻击模式

## 使用方法

### 基本使用

```bash
cd docs
node ips.js
```

### 指定日志目录

```bash
# 使用 --logs-dir 或 -d 参数指定日志目录
node ips.js --logs-dir /var/log/nginx
node ips.js -d ./custom-logs

# 或者直接使用位置参数
node ips.js /var/log/nginx
```

### 查看帮助

```bash
node ips.js --help
# 或
node ips.js -h
```

### 输出说明

脚本会输出：

1. **分析进度**: 显示正在分析的日志文件
2. **详细统计**: 每个异常 IP 的详细信息
   - IP 地址
   - 异常请求次数
   - 异常请求路径列表
3. **汇总统计**: 
   - 总异常 IP 数
   - 总异常请求次数
   - 前 10 个最活跃的异常 IP
4. **防火墙规则**: 自动生成可用于防火墙的规则

### 输出示例

```
日志目录: /path/to/docs/logs
找到 2 个日志文件，开始分析...

分析文件: 1.log
分析文件: 2.log

================================================================================
异常 IP 分析结果
================================================================================

共发现 15 个异常 IP
总异常请求次数: 234

前 10 个最活跃的异常 IP:
  1. 20.196.91.24 - 45 次异常请求
  2. 18.182.37.84 - 38 次异常请求
  3. 38.148.244.2 - 12 次异常请求
  ...

================================================================================
异常 IP 列表（逗号分隔）
================================================================================
20.196.91.24, 18.182.37.84, 38.148.244.2, 139.59.78.17, 122.192.32.40, ...
```

## 应用防火墙规则

脚本会输出逗号分隔的异常 IP 列表，你可以：

### 1. 复制 IP 列表

直接复制输出的逗号分隔 IP 列表，用于：
- 手动添加到防火墙规则
- 导入到安全工具
- 添加到黑名单

### 2. 生成 Nginx deny 规则

```bash
# 使用脚本输出生成 Nginx 规则
node ips.js | grep -A 1000 "异常 IP 列表" | tail -n +4 | tr ',' '\n' | sed 's/^/deny /;s/$/;/' > nginx-deny.conf
```

### 3. 生成 iptables 规则

```bash
# 使用脚本输出生成 iptables 规则
node ips.js | grep -A 1000 "异常 IP 列表" | tail -n +4 | tr ',' '\n' | sed 's/^/iptables -A INPUT -s /;s/$/ -j DROP/' > iptables-rules.sh
```

### 4. 使用 fail2ban

也可以将异常 IP 添加到 fail2ban 的 ban 列表中。

## 自定义规则

如果需要添加自定义的异常路径规则，编辑 `ips.js` 文件：

1. 在 `SUSPICIOUS_PATHS` 数组中添加精确路径
2. 在 `SUSPICIOUS_PATTERNS` 数组中添加正则表达式模式

## 注意事项

1. **误判**: 某些正常请求可能被误判为异常（如正常的 `/admin` 路径），需要根据实际情况调整规则
2. **日志格式**: 脚本针对 Nginx 错误日志格式编写，其他格式的日志可能需要调整解析逻辑
3. **性能**: 对于大型日志文件，分析可能需要一些时间

## 定期运行

建议定期运行此脚本（如每天或每周），及时发现新的异常 IP：

```bash
# 添加到 crontab
0 2 * * * cd /path/to/docs && node ips.js >> /var/log/ip-analysis.log 2>&1
```

