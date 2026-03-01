
## 📋 核心目标

将 oic-cache 改造为**独立服务**，支持两种协议：
- **Redis Protocol**（6379端口）：给 oic-web 用，高频访问
- **gRPC**（50051端口）：给 oic-admin 用，管理功能

---

## 🏗️ 整体架构

```
         Redis客户端            gRPC客户端
         (oic-web)            (oic-admin)
              ↓                    ↓
         Redis协议              gRPC协议
         (6379端口)            (50051端口)
              ↓                    ↓
         ┌────────────────────────────┐
         │   oic-cache-server         │
         │   (独立进程)               │
         │                            │
         │   Core Cache (现有实现)    │
         └────────────────────────────┘
```

---

## 🔧 技术选型

### 1. Redis Protocol 实现

#### 选用库：**手动实现**（无需额外库）

**原因：**
- RESP 协议很简单（只是文本格式）
- 自己实现更灵活，能支持自定义命令
- 避免依赖外部库的限制

**RESP 协议格式：**
```
客户端发送：*3\r\n$3\r\nSET\r\n$4\r\nkey1\r\n$5\r\nvalue\r\n
           (数组：[SET, key1, value])

服务端返回：+OK\r\n
           (简单字符串：OK)
```

**实现位置：** `src/server/resp.rs`

**核心函数：**
```rust
// 解析一条 RESP 数组，返回参数列表与消费字节数
fn parse_resp(data: &[u8]) -> Result<(Vec<Vec<u8>>, usize), RespError>;

// 编码服务端响应
fn encode_simple_string(s: &str) -> Vec<u8>;   // +OK\r\n
fn encode_bulk_string(data: &[u8]) -> Vec<u8>;  // $5\r\nhello\r\n
fn encode_null_bulk_string() -> Vec<u8>;        // $-1\r\n
fn encode_integer(n: i64) -> Vec<u8>;           // :123\r\n
fn encode_error(msg: &str) -> Vec<u8>;         // -ERR ...\r\n
```

**依赖：**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
bytes = "1.5"
```

---

### 2. gRPC 实现

#### 选用库：**tonic**

**原因：**
- Rust 最成熟的 gRPC 库
- 纯 Rust 实现，性能好
- 异步支持（基于 Tokio）

**依赖：**
```toml
[dependencies]
tonic = "0.12"
prost = "0.13"  # Protobuf 序列化（tonic 自带）

[build-dependencies]
tonic-build = "0.12"  # 从 .proto 生成 Rust 代码
```

**核心流程：**
1. 写 `proto/cache.proto` 文件（定义接口）
2. `tonic-build` 自动生成 Rust 代码
3. 实现生成的 trait

---

## 📦 项目结构

```
oic-cache/
├── proto/
│   └── cache.proto              # gRPC 接口定义
├── build.rs                     # tonic-build 编译 proto
├── src/
│   ├── lib.rs                   # 现有缓存核心 + pub mod server
│   └── server/                  # 新增
│       ├── mod.rs
│       ├── resp.rs              # RESP 解析与编码
│       ├── redis_server.rs      # Redis 协议服务器
│       └── grpc_server.rs       # gRPC 服务器
├── bin/
│   └── oic-cache-server.rs      # 服务器启动入口
└── Cargo.toml
```

---

## 🎯 实现步骤

### 步骤 1：Redis Protocol 服务器

**核心逻辑：** `src/server/redis_server.rs`

- `RedisServer::new(cache: Arc<Cache>)` 创建服务，`run(addr: &str)` 绑定并循环 accept，每连接 spawn `handle_client`。
- 使用 `resp::parse_resp` 解析 RESP 数组，得到 `(args, consumed)`，按命令分发到 `dispatch()`。
- `SET` 支持可选 `EX seconds`；未指定时使用默认 TTL。value 的 content-type 固定为 `application/octet-stream`。
- 命令与 Cache API 对应：GET→`get`，SET→`set_with_ttl`，DEL→`invalidate`，EXISTS→`exists`，FLUSHALL→`clear`，INVALIDATE_NS→`invalidate_namespace`，STATS→`statistics()`。

**支持的命令：**
- `GET key`
- `SET key value [EX seconds]`
- `DEL key`
- `EXISTS key`
- `FLUSHALL`
- `PING`

**扩展命令（自定义）：**
- `INVALIDATE_NS namespace`
- `STATS`

---

### 步骤 2：gRPC 服务器

**接口定义：** `proto/cache.proto`

```protobuf
syntax = "proto3";

package cache;

service CacheService {
  rpc Get(GetRequest) returns (GetResponse);
  rpc Set(SetRequest) returns (SetResponse);
  rpc GetStatistics(Empty) returns (StatsResponse);
  rpc InvalidateNamespace(InvalidateRequest) returns (InvalidateResponse);
}

message GetRequest { string key = 1; }
message GetResponse { bytes data = 1; bool found = 2; }

message SetRequest {
  string key = 1;
  bytes data = 2;
  int64 ttl_seconds = 3;  // 0 表示使用默认 TTL
}

message SetResponse { bool success = 1; }
message Empty {}

message StatsResponse {
  int64 total_requests = 1;
  int64 hits = 2;
  int64 misses = 3;
  double hit_rate = 4;
  int64 total_entries = 5;
}

message InvalidateRequest { string namespace = 1; }
message InvalidateResponse { int32 invalidated_count = 1; }
```

**服务实现：** `src/server/grpc_server.rs`

- 使用 `mod proto { tonic::include_proto!("cache"); }` 引入生成代码，实现 `proto::cache_service_server::CacheService`。
- `Get` → `cache.get()`，`Set` → `cache.set_with_ttl(..., "application/octet-stream", ttl)`（ttl_seconds 为 0 时用默认 360）。
- `GetStatistics` → `cache.statistics()` 填入 total_requests / hits / misses / hit_rate / total_entries。
- `InvalidateNamespace` → `cache.invalidate_namespace()`，返回 `InvalidateResponse { invalidated_count }`。
- `start_grpc_server(cache: Arc<Cache>, addr: SocketAddr)` 启动 gRPC 服务。

---

### 步骤 3：服务器启动入口

**入口：** `bin/oic-cache-server.rs`

- 使用 `tracing_subscriber` 初始化日志；通过环境变量 `OIC_CACHE_CONFIG` 指定配置文件路径（TOML），未设置则使用 `CacheConfig::default()`。
- `Cache::new(config)` 为同步构造；随后调用 `cache.load_index().await` 尝试从磁盘加载索引（无文件则忽略）。
- 用 `Arc::new(cache)` 共享给两个服务；Redis 服务在 `tokio::spawn` 中执行 `RedisServer::new(redis_cache).run(&redis_addr).await`；gRPC 使用 `start_grpc_server(cache, grpc_addr).await`，地址类型为 `SocketAddr`。
- 环境变量：`OIC_CACHE_REDIS_ADDR`（默认 `0.0.0.0:6379`）、`OIC_CACHE_GRPC_ADDR`（默认 `0.0.0.0:50051`）。

---

## 🚀 使用方式

### 启动服务器

```bash
cargo build --release --bin oic-cache-server
./target/release/oic-cache-server
```

### 客户端使用

#### oic-web（Redis 客户端）

```rust
// 使用标准 Redis 客户端库
use redis::Commands;

let client = redis::Client::open("redis://127.0.0.1:6379")?;
let mut conn = client.get_connection()?;

// 像使用 Redis 一样
conn.set_ex("user:123", b"alice", 3600)?;
let value: Option<Vec<u8>> = conn.get("user:123")?;
```

#### oic-admin（gRPC 客户端）

使用 `tonic` 从同一 `proto/cache.proto` 生成客户端后：

```rust
use cache::cache_service_client::CacheServiceClient;
use cache::Empty;

let mut client = CacheServiceClient::connect("http://127.0.0.1:50051").await?;
let stats = client.get_statistics(Empty {}).await?;
println!("Hit rate: {}", stats.into_inner().hit_rate);
```

---

## 📝 完整依赖清单

在 `crates/oic-cache/Cargo.toml` 中：

```toml
[dependencies]
# ... 现有依赖
tonic = "0.12"
prost = "0.13"

[build-dependencies]
tonic-build = "0.12"

[[bin]]
name = "oic-cache-server"
path = "bin/oic-cache-server.rs"
```

`build.rs` 中调用 `tonic_build::compile_protos("proto/cache.proto")`。

---

## 📋 核心概念

### 什么是"服务化"？

**现在：** oic-cache 是一个库（library），需要编译到你的程序里
```
oic-web 程序 = 你的代码 + oic-cache 库
oic-admin 程序 = 你的代码 + oic-cache 库
```
- 问题：两个程序各有各的缓存，不共享

**改造后：** oic-cache 是一个独立进程（server），通过网络访问
```
oic-web ──网络通信──▶ oic-cache-server ◀──网络通信── oic-admin
```
- 优势：一个缓存实例，多个程序共享

---

## 🎯 技术目标

实现两个"监听端口"：

### 1. Redis Protocol（端口 6379）
- **作用：** 数据访问接口
- **用户：** oic-web（高频读写缓存）
- **原理：** 实现 Redis 的通信协议，让任何 Redis 客户端都能连接

### 2. gRPC（端口 50051）
- **作用：** 管理接口
- **用户：** oic-admin（查看统计、清空缓存等）
- **原理：** 使用 Google 的 RPC 框架，类型安全、功能丰富

---

## 🔍 Redis Protocol 详解

### 什么是 Redis Protocol？

Redis 使用一种叫 **RESP (Redis Serialization Protocol)** 的文本协议。

**举例说明：**

客户端想执行 `SET name "Alice"`：
```
发送的数据：
*3\r\n
$3\r\nSET\r\n
$4\r\nname\r\n
$5\r\nAlice\r\n

解释：
*3        → 数组有 3 个元素
$3\r\nSET → 第一个元素："SET"（3个字节）
$4\r\nname → 第二个元素："name"（4个字节）
$5\r\nAlice → 第三个元素："Alice"（5个字节）
```

服务端返回成功：
```
+OK\r\n

解释：
+ → 简单字符串
OK → 内容
\r\n → 结束符
```

### 为什么选择 Redis Protocol？

1. **生态完善：** 所有编程语言都有现成的 Redis 客户端
2. **无需写客户端：** oic-web 直接用 `redis-rs` 库连接
3. **工具丰富：** 可以用 `redis-cli` 测试你的服务器
4. **迁移简单：** 用户从 Redis 切换到 oic-cache 无需改代码

### 需要实现什么？

**核心任务：** 写一个 TCP 服务器，能够：
1. 监听 6379 端口
2. 接收客户端发来的 RESP 格式数据
3. 解析出命令（如 GET、SET）
4. 调用你现有的 `Cache` 执行操作
5. 把结果编码成 RESP 格式返回

**技术组件：**
- **TCP 服务器：** 用 `tokio::net::TcpListener`（Tokio 提供的异步 TCP）
- **协议解析：** 自己写解析函数（RESP 协议很简单，100行代码搞定）
- **并发处理：** Tokio 的 `spawn`（每个连接一个异步任务）

**不需要额外库：** 只用 `tokio` + `bytes`（处理字节数组）

---

## 🔍 gRPC 详解

### 什么是 gRPC？

gRPC 是 Google 开发的 **RPC (Remote Procedure Call)** 框架。

**核心思想：** 让远程调用像本地函数调用一样简单。

**传统方式（HTTP）：**
```
1. 构造 JSON：{"key": "user:123"}
2. 发送 HTTP POST
3. 解析 JSON 响应
4. 处理错误
```

**gRPC 方式：**
```
直接调用：response = client.get("user:123")
（底层自动处理序列化、网络传输、反序列化）
```

### gRPC 的核心概念

#### 1. Protobuf（协议定义）

用 `.proto` 文件定义接口：
```protobuf
service CacheService {
  rpc Get(GetRequest) returns (GetResponse);
}

message GetRequest {
  string key = 1;
}

message GetResponse {
  bytes data = 1;
  bool found = 2;
}
```

**作用：**
- 定义"合同"（服务端提供什么接口）
- 跨语言（Java、Go、Rust 都能用同一个 proto 文件）
- 自动生成代码

#### 2. 编译过程

```
proto/cache.proto 
    ↓ (tonic-build)
自动生成的 Rust 代码：
  - CacheService trait（你要实现的接口）
  - CacheServiceServer（服务端）
  - CacheServiceClient（客户端）
  - 消息类型（GetRequest, GetResponse）
```

#### 3. 传输层

- **协议：** HTTP/2（多路复用、流式传输）
- **序列化：** Protocol Buffers（比 JSON 快、体积小）
- **类型检查：** 编译时就能发现错误

### 为什么用 gRPC？

1. **强类型：** 编译时检查，不会出现"字段名拼错"的问题
2. **高性能：** Protobuf 比 JSON 快 3-10 倍
3. **流式传输：** 支持服务端推送、双向流
4. **功能丰富：** 适合复杂的管理接口

### 需要实现什么？

**核心任务：**
1. 写 `.proto` 文件定义接口
2. `tonic-build` 自动生成 Rust 代码
3. 实现生成的 trait（把 gRPC 请求转换为 Cache 调用）
4. 启动 gRPC 服务器

**技术组件：**
- **gRPC 框架：** `tonic`（Rust 最成熟的 gRPC 库）
- **序列化库：** `prost`（Protobuf 的 Rust 实现）
- **代码生成：** `tonic-build`（编译时从 .proto 生成代码）

---

## 📚 技术选型总结

### 核心依赖库

| 库名 | 作用 | 为什么选它 |
|------|------|-----------|
| **tokio** | 异步运行时 | Rust 异步标准，性能最好 |
| **tonic** | gRPC 框架 | 纯 Rust，功能完整，生态好 |
| **prost** | Protobuf 序列化 | tonic 的配套库 |
| **bytes** | 字节处理 | 高效处理网络数据 |

### 不需要的库

| 可能考虑的库 | 为什么不用 |
|------------|-----------|
| `redis-protocol` | RESP 协议简单，自己实现更灵活 |
| `mini-redis` | 完整的 Redis 实现，我们只需要协议部分 |
| `tarpc` | gRPC 已经够用，不需要另一个 RPC 框架 |
| `actix-web` | 不需要 HTTP 服务器 |

---

## 🏗️ 架构分层

### 层次结构

```
┌────────────────────────────────────────┐
│     网络层（监听端口）                   │
│  ┌──────────────┐  ┌──────────────┐   │
│  │ Redis Server │  │  gRPC Server │   │
│  │  (6379)      │  │   (50051)    │   │
│  └──────┬───────┘  └──────┬───────┘   │
└─────────┼──────────────────┼───────────┘
          │                  │
┌─────────┼──────────────────┼───────────┐
│     协议层（解析和编码）                 │
│  ┌──────▼───────┐  ┌──────▼───────┐   │
│  │ RESP Parser  │  │Protobuf Codec│   │
│  │ (手动实现)    │  │ (tonic 提供) │   │
│  └──────┬───────┘  └──────┬───────┘   │
└─────────┼──────────────────┼───────────┘
          │                  │
┌─────────┼──────────────────┼───────────┐
│     业务层（命令处理）                   │
│  ┌──────▼───────┐  ┌──────▼───────┐   │
│  │Redis Handler │  │ gRPC Handler │   │
│  │(GET/SET/DEL) │  │(管理接口)     │   │
│  └──────┬───────┘  └──────┬───────┘   │
└─────────┼──────────────────┼───────────┘
          │                  │
          └──────────┬───────┘
                     ▼
          ┌────────────────┐
          │  Core Cache    │
          │  (现有实现)     │
          └────────────────┘
```

### 各层职责

| 层 | 职责 | 技术 |
|----|------|------|
| **网络层** | 监听端口、接受连接 | `tokio::net::TcpListener` + `tonic::transport::Server` |
| **协议层** | 数据格式转换 | 自己写 RESP 解析 + tonic 自动处理 Protobuf |
| **业务层** | 命令分发、参数验证 | 匹配命令类型，调用 Cache |
| **核心层** | 缓存逻辑 | 你现有的 Cache 代码（不改动） |

---

## 🔄 工作流程

### Redis Protocol 流程

```
1. 客户端连接 → TcpListener.accept()
2. 读取字节流 → TcpStream.read()
3. 解析 RESP → parse_resp() 返回 ["GET", "key1"]
4. 匹配命令 → match args[0] { "GET" => ... }
5. 调用缓存 → cache.get("key1")
6. 编码响应 → encode_bulk_string(data)
7. 发送结果 → TcpStream.write()
```

### gRPC 流程

```
1. 客户端调用 → client.get(GetRequest { key: "key1" })
2. tonic 自动处理 → HTTP/2 + Protobuf 编码
3. 服务端接收 → CacheService::get()
4. 调用缓存 → self.cache.get("key1")
5. 构造响应 → GetResponse { data, found }
6. tonic 自动处理 → Protobuf 编码 + HTTP/2
7. 返回客户端 → Response<GetResponse>
```

---

## 🎓 关键技术点

### 1. 异步编程（Tokio）

**为什么用异步？**
- 单线程处理上万个连接
- 高并发场景性能好

**核心概念：**
- `async fn` - 异步函数
- `.await` - 等待异步操作完成
- `tokio::spawn` - 启动新的异步任务

### 2. RESP 协议解析

**核心知识点：**
- 文本协议（可读性好）
- 5 种数据类型：`+`简单字符串、`-`错误、`:`整数、`$`批量字符串、`*`数组
- `\r\n` 作为分隔符

**实现难度：** 低（状态机解析，约 100-200 行代码）

### 3. Protobuf 编译

**工作原理：**
- `build.rs` 在编译时运行
- `tonic-build` 读取 `.proto` 文件
- 生成 Rust 代码到 `src/generated/` 或 `target/` 目录
- 编译到最终程序

**优势：** 完全自动化，你只需写 `.proto` 文件

### 4. 并发安全

**需要注意：**
- `Cache` 需要 `Arc` 包装（多个连接共享）
- `Cache` 内部要线程安全（你已经用了 `DashMap`，没问题）

---

## 📊 性能考虑

### Redis Protocol
- **延迟：** ~100 微秒（网络 + 解析 + 缓存操作）
- **吞吐量：** 10万+ QPS（单机）
- **瓶颈：** 网络 I/O

### gRPC
- **延迟：** ~200 微秒（比 Redis 稍慢，因为 HTTP/2 开销）
- **吞吐量：** 5万+ QPS
- **优势：** 复杂数据结构传输快

---

## ✅ 总结

### 核心技术栈

| 组件 | 技术选择 | 理由 |
|------|---------|------|
| **异步运行时** | Tokio | 行业标准 |
| **Redis 协议** | 手动实现 | 简单、灵活 |
| **gRPC 框架** | Tonic | 成熟、高性能 |
| **序列化** | Protobuf (prost) | 快速、跨语言 |

### 工作量估算

| 任务 | 难度 | 时间 |
|------|------|------|
| **RESP 协议解析** | 中 | 1-2天 |
| **Redis 命令处理** | 低 | 1天 |
| **Protobuf 定义** | 低 | 0.5天 |
| **gRPC 服务实现** | 低 | 1天 |

---

## 当前实现摘要

- **proto**：`crates/oic-cache/proto/cache.proto`，package `cache`，含 Get/Set/GetStatistics/InvalidateNamespace 及 Empty、InvalidateResponse、StatsResponse 等消息。
- **RESP**：`src/server/resp.rs`，`parse_resp` 返回 `(Vec<Vec<u8>>, consumed)`，编码函数 `encode_*`。
- **Redis 服务**：`src/server/redis_server.rs`，`RedisServer::new(Arc<Cache>)`、`run(addr)`，支持 GET/SET/DEL/EXISTS/FLUSHALL/PING/INVALIDATE_NS/STATS，SET 支持 `EX seconds`。
- **gRPC 服务**：`src/server/grpc_server.rs`，`CacheServiceImpl` 实现生成的 `CacheService`，`start_grpc_server(cache, SocketAddr)`。
- **入口**：`bin/oic-cache-server.rs`，环境变量 `OIC_CACHE_CONFIG`、`OIC_CACHE_REDIS_ADDR`、`OIC_CACHE_GRPC_ADDR`，启动时执行 `load_index()`。