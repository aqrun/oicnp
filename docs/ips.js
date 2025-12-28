#!/usr/bin/env node

/**
 * 日志分析脚本 - 分析异常 IP
 * 从 logs 目录下的所有日志文件中提取异常 IP 地址
 */

const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

/**
 * 解析命令行参数
 */
function parseArgs() {
  const args = process.argv.slice(2);
  const config = {
    logsDir: path.join(__dirname, 'logs'), // 默认值
  };
  
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '--logs-dir' || arg === '-d') {
      if (i + 1 < args.length) {
        config.logsDir = path.resolve(args[i + 1]);
        i++;
      } else {
        console.error(`错误: ${arg} 需要指定目录路径`);
        process.exit(1);
      }
    } else if (arg === '--help' || arg === '-h') {
      console.log(`
用法: node ips.js [选项]

选项:
  -d, --logs-dir <目录>  指定日志文件目录 (默认: ./logs)
  -h, --help             显示帮助信息

示例:
  node ips.js
  node ips.js --logs-dir /var/log/nginx
  node ips.js -d ./custom-logs
      `);
      process.exit(0);
    } else if (!arg.startsWith('-')) {
      // 如果参数不是选项，可能是位置参数（日志目录）
      config.logsDir = path.resolve(arg);
    }
  }
  
  return config;
}

// 异常路径规则（常见攻击路径）
const SUSPICIOUS_PATHS = [
  // 配置文件
  '/package.json',
  '/package-lock.json',
  '/yarn.lock',
  '/composer.json',
  '/composer.lock',
  '/pom.xml',
  '/requirements.txt',
  '/.env',
  '/.env.local',
  '/.env.production',
  '/.env.development',
  '/config',
  '/config.js',
  '/config.php',
  '/config.py',
  '/config.json',
  '/settings.py',
  '/settings.js',
  '/.config',
  
  // 版本控制
  '/.git',
  '/.git/config',
  '/.git/HEAD',
  '/.git/refs',
  '/.github',
  '/.gitlab',
  '/.svn',
  '/.hg',
  
  // WordPress 相关
  '/wp-admin',
  '/wp-content',
  '/wp-includes',
  '/wp-login.php',
  '/wp-config.php',
  '/xmlrpc.php',
  
  // 管理后台
  '/admin',
  '/admin.php',
  '/administrator',
  '/backend',
  '/manage',
  '/management',
  
  // 敏感文件
  '/.htaccess',
  '/.htpasswd',
  '/web.config',
  '/.well-known',
  '/.DS_Store',
  '/Thumbs.db',
  
  // PHP 相关（常见漏洞扫描）
  '/phpinfo.php',
  '/info.php',
  '/test.php',
  '/shell.php',
  '/cmd.php',
  '/eval.php',
  '/1.php',
  '/a.php',
  '/aa.php',
  '/abcd.php',
  '/admin.php',
  '/api.php',
  '/bolt.php',
  '/bless.php',
  '/alfa.php',
  
  // 其他敏感路径
  '/.netlify',
  '/.vercel',
  '/.next',
  '/node_modules',
  '/vendor',
  '/storage',
  '/secrets',
  '/keys',
  '/credentials',
  '/stripe',
  '/payment',
  '/webhook',
];

// 异常路径模式（正则表达式）
const SUSPICIOUS_PATTERNS = [
  /\.php$/i,           // 所有 .php 文件
  /\.env/i,            // 环境变量文件
  /\.git/i,            // Git 相关
  /wp-/i,              // WordPress
  /admin/i,            // 管理后台
  /config/i,           // 配置文件
  /secret/i,           // 密钥相关
  /key/i,              // 密钥
  /credential/i,       // 凭证
  /password/i,         // 密码
  /\.well-known/i,     // 已知路径
  /autoload/i,         // 自动加载
  /\.(jar|war|ear)$/i, // Java 打包文件
  /\+CSCO/i,           // Cisco 相关
  /HNAP/i,             // HNAP 协议
  /nmaplowercheck/i,   // Nmap 扫描
  /sdk/i,              // SDK
  /evox/i,             // Evox
];

/**
 * 判断请求路径是否为异常
 */
function isSuspiciousRequest(requestPath) {
  if (!requestPath) return false;
  
  // 检查精确匹配
  for (const suspiciousPath of SUSPICIOUS_PATHS) {
    if (requestPath === suspiciousPath || requestPath.startsWith(suspiciousPath + '/')) {
      return true;
    }
  }
  
  // 检查模式匹配
  for (const pattern of SUSPICIOUS_PATTERNS) {
    if (pattern.test(requestPath)) {
      return true;
    }
  }
  
  return false;
}

/**
 * 解析日志行，提取 IP 和请求路径
 */
function parseLogLine(line) {
  // Nginx 错误日志格式：
  // 2025/12/27 12:22:26 [error] 687824#687824: *44668 connect() failed (111: Connection refused) while connecting to upstream, client: 38.148.244.2, server: lxage.com, request: "POST /path HTTP/1.1", ...
  
  const clientMatch = line.match(/client:\s+([\d.]+)/);
  
  // 支持带引号和不带引号的请求格式
  const requestMatch = line.match(/request:\s+"?([A-Z]+)\s+([^\s"]+)\s+HTTP\/[\d.]+"?/);
  
  if (!clientMatch || !requestMatch) {
    return null;
  }
  
  const ip = clientMatch[1];
  const method = requestMatch[1];
  let path = requestMatch[2];
  
  // 清理路径（移除查询参数和锚点）
  path = path.split('?')[0].split('#')[0];
  
  return { ip, method, path };
}

/**
 * 读取文件内容（支持 .gz 压缩文件）
 */
function readFileContent(filePath) {
  if (filePath.endsWith('.gz')) {
    // 读取并解压 .gz 文件
    const buffer = fs.readFileSync(filePath);
    return zlib.gunzipSync(buffer).toString('utf-8');
  } else {
    // 读取普通文件
    return fs.readFileSync(filePath, 'utf-8');
  }
}

/**
 * 分析单个日志文件
 */
function analyzeLogFile(filePath) {
  let content;
  try {
    content = readFileContent(filePath);
  } catch (error) {
    console.error(`  警告: 无法读取文件 ${path.basename(filePath)}: ${error.message}`);
    return new Map();
  }
  
  const lines = content.split('\n');
  const ipStats = new Map(); // IP -> { count, requests: [] }
  
  for (const line of lines) {
    if (!line.trim()) continue;
    
    const parsed = parseLogLine(line);
    if (!parsed) continue;
    
    const { ip, method, path } = parsed;
    
    // 判断是否为异常请求
    if (isSuspiciousRequest(path)) {
      if (!ipStats.has(ip)) {
        ipStats.set(ip, {
          count: 0,
          requests: []
        });
      }
      
      const stats = ipStats.get(ip);
      stats.count++;
      
      // 记录请求（去重）
      const requestKey = `${method} ${path}`;
      if (!stats.requests.includes(requestKey)) {
        stats.requests.push(requestKey);
      }
    }
  }
  
  return ipStats;
}

/**
 * 判断文件是否为日志文件
 */
function isLogFile(fileName) {
  // 支持的文件类型：
  // 1. .log 结尾的文件（如 error.log, access.log）
  // 2. .log-YYYYMMDD 格式的文件（如 admin_lxage_error.log-20251228）
  // 3. .log-YYYYMMDD.gz 格式的压缩文件（如 admin_lxage_error.log-20251219.gz）
  // 4. 包含 log 关键字的 .gz 压缩文件（如 api_lxage_access.log-20251220.gz）
  
  const lowerName = fileName.toLowerCase();
  
  // .log 结尾（包括 .log-YYYYMMDD 格式）
  if (lowerName.endsWith('.log') || /\.log-\d{8}$/.test(lowerName)) {
    return true;
  }
  
  // .gz 压缩文件（包含 log 关键字）
  if (lowerName.endsWith('.gz') && lowerName.includes('log')) {
    return true;
  }
  
  return false;
}

/**
 * 分析所有日志文件
 */
function analyzeAllLogs(logsDir) {
  const allFiles = fs.readdirSync(logsDir);
  const files = allFiles
    .filter(file => {
      const filePath = path.join(logsDir, file);
      const stats = fs.statSync(filePath);
      // 只处理文件，不处理目录
      return stats.isFile() && isLogFile(file);
    })
    .map(file => path.join(logsDir, file))
    .sort(); // 按文件名排序
  
  if (files.length === 0) {
    console.error('未找到日志文件！');
    console.error(`提示: 支持的日志文件格式包括: .log, .log-YYYYMMDD, .log-YYYYMMDD.gz, *.log*.gz`);
    process.exit(1);
  }
  
  console.log(`找到 ${files.length} 个日志文件，开始分析...\n`);
  
  // 合并所有文件的统计
  const allIpStats = new Map();
  
  for (const file of files) {
    const fileName = path.basename(file);
    const fileType = fileName.endsWith('.gz') ? ' (压缩)' : '';
    console.log(`分析文件: ${fileName}${fileType}`);
    
    const fileStats = analyzeLogFile(file);
    
    // 合并统计
    for (const [ip, stats] of fileStats.entries()) {
      if (!allIpStats.has(ip)) {
        allIpStats.set(ip, {
          count: 0,
          requests: []
        });
      }
      
      const existing = allIpStats.get(ip);
      existing.count += stats.count;
      
      // 合并请求列表（去重）
      for (const req of stats.requests) {
        if (!existing.requests.includes(req)) {
          existing.requests.push(req);
        }
      }
    }
  }
  
  return allIpStats;
}

/**
 * 输出分析结果
 */
function outputResults(ipStats) {
  // 按异常请求次数排序
  const sortedIps = Array.from(ipStats.entries())
    .sort((a, b) => b[1].count - a[1].count);
  
  console.log('\n' + '='.repeat(80));
  console.log('异常 IP 分析结果');
  console.log('='.repeat(80));
  console.log(`\n共发现 ${sortedIps.length} 个异常 IP`);
  console.log(`总异常请求次数: ${Array.from(ipStats.values()).reduce((sum, s) => sum + s.count, 0)}`);
  
  // 输出前 10 个最活跃的异常 IP
  if (sortedIps.length > 0) {
    console.log('\n前 10 个最活跃的异常 IP:');
    sortedIps.slice(0, 10).forEach(([ip, stats], index) => {
      console.log(`  ${index + 1}. ${ip} - ${stats.count} 次异常请求`);
    });
  }
  
  // 输出逗号分隔的 IP 列表
  console.log('\n' + '='.repeat(80));
  console.log('异常 IP 列表（逗号分隔）');
  console.log('='.repeat(80));
  if (sortedIps.length > 0) {
    const ipList = sortedIps.map(([ip]) => ip).join(', ');
    console.log(ipList);
  } else {
    console.log('未发现异常 IP');
  }
}

/**
 * 主函数
 */
function main() {
  try {
    const config = parseArgs();
    const logsDir = config.logsDir;
    
    if (!fs.existsSync(logsDir)) {
      console.error(`错误: 日志目录不存在: ${logsDir}`);
      process.exit(1);
    }
    
    const stats = fs.statSync(logsDir);
    if (!stats.isDirectory()) {
      console.error(`错误: ${logsDir} 不是一个目录`);
      process.exit(1);
    }
    
    console.log(`日志目录: ${logsDir}`);
    const ipStats = analyzeAllLogs(logsDir);
    outputResults(ipStats);
    
  } catch (error) {
    console.error('分析过程中出错:', error);
    process.exit(1);
  }
}

// 运行脚本
if (require.main === module) {
  main();
}

module.exports = { 
  analyzeAllLogs, 
  isSuspiciousRequest, 
  parseLogLine, 
  parseArgs,
  isLogFile,
  readFileContent
};

