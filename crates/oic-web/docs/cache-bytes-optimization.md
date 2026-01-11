# Cache Service Bytes 返回类型优化方案

## 背景

当前 `get_cached_or_render` 函数虽然内部使用 `Bytes` 进行缓存（零拷贝），但返回类型是 `String`，导致每次返回都需要进行 `Bytes` → `Vec<u8>` → `String` 的转换，存在不必要的性能开销。

**优化目标**：直接返回 `Bytes` 类型，在 Controller 层使用 `Html<Bytes>` 作为响应，实现真正的零拷贝。

**技术基础**：Axum 原生支持 `Html<Bytes>`，可以直接将 `Bytes` 作为 HTML 响应，自动设置 `Content-Type: text/html`，无需任何类型转换。

**架构设计：分离数据获取和渲染**：
- **设计思想**：分离「数据获取（异步）」和「模板渲染（同步）」，解决 Rust 闭包无法捕获 async 上下文的问题
- **实现方式**：`render_fn` 参数接收一个异步闭包，内部包含数据获取和渲染逻辑
- **数据流**：
  1. **数据获取（异步）**：在 View 层（如 `render_home_index`）异步获取数据（调用 API、数据库查询等）
  2. **模板渲染（同步）**：在 Controller 的 `render_fn` 中同步渲染 Askama 模板
  3. **类型转换**：将渲染后的 `String` 转换为 `Bytes`
- **优势**：清晰分离异步数据获取和同步模板渲染，符合 Rust 的异步编程模型

参考：[Cache Askama 集成方案 - 分离数据获取和渲染](../../../docs/cache-askama.md#三终极方案分离数据获取和渲染-)

## Rust 缓存方案对比

基于传统 Web 框架的最佳实践（缓存都在 Controller 层处理），以下是适合 Rust/Axum 的三种缓存方案：

### 方案 1：属性宏（最接近 Spring @Cacheable）⭐⭐⭐⭐⭐

#### 实现思路

使用过程宏（proc-macro）在 Controller handler 函数上添加属性，类似 Spring 的 `@Cacheable` 注解。

#### 代码示例

```rust
// 使用属性宏（类似 Spring @Cacheable）
#[cacheable(key = "home:index", ttl = 3600)]
async fn index(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let bytes = render_home_index(manifest.clone()).await?;
    Html(bytes).into_response()
}

// 动态 key
#[cacheable(key_fn = "format!(\"blog:{}\", vid)", ttl = 7200)]
async fn blog_detail(
    Path(vid): Path<String>,
    Extension(cache): Extension<CacheExtension>,
    Extension(manifest): Extension<ManifestExtension>,
) -> impl IntoResponse {
    let bytes = render_blog_detail(vid, manifest.clone()).await?;
    Html(bytes).into_response()
}
```

#### 实现要求

需要创建一个 proc-macro crate：

```rust
// oic-web-macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs};

#[proc_macro_attribute]
pub fn cacheable(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);
    
    // 解析 key 和 ttl
    // 生成缓存逻辑包装代码
    // ...
}
```

#### 优点

- ✅ **声明式**：在函数上标注，代码最简洁
- ✅ **零成本抽象**：编译时展开，无运行时开销
- ✅ **类型安全**：编译期检查
- ✅ **高度灵活**：支持动态 key（通过 key_fn）

#### 缺点

- ⚠️ **需要 proc-macro crate**：需要单独创建和维护
- ⚠️ **编译时间增加**：过程宏会增加编译时间
- ⚠️ **调试相对困难**：宏展开后的代码不易调试
- ⚠️ **实现复杂度高**：需要处理 Axum handler 的复杂签名

#### 适用场景

- 长期维护的项目
- 追求最优雅的代码风格
- 团队有宏开发经验

---

### 方案 2：Tower Middleware（类似 Go/PHP）⭐⭐⭐⭐

#### 实现思路

使用 Tower 中间件在请求管道中拦截，类似 Go 的 Gin/Echo 和 PHP 的 Laravel/Symfony。

#### 代码示例

```rust
// 定义缓存中间件
use tower::Layer;

pub fn cache_middleware<S>(cache: Arc<Cache>) -> CacheLayer<S> {
    CacheLayer::new(cache)
}

// Router 配置
pub fn home_routes() -> Router<AppContext> {
    Router::new()
        .route("/", get(index))
        .layer(cache_middleware(cache.clone()))
}

// Handler（业务逻辑，缓存由中间件处理）
async fn index(
    Extension(manifest): Extension<ManifestExtension>,
) -> impl IntoResponse {
    let bytes = render_home_index(manifest).await?;
    Html(bytes).into_response()
}
```

#### 实现示例

```rust
// src/middleware/cache.rs
use axum::{
    extract::Request,
    response::Response,
};
use tower::Service;
use std::sync::Arc;
use oic_cache::Cache;

pub struct CacheLayer<S> {
    cache: Arc<Cache>,
    inner: S,
}

impl<S> CacheLayer<S> {
    pub fn new(cache: Arc<Cache>) -> Self {
        Self { cache, inner: todo!() }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CacheLayer<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = /* ... */;

    fn call(&self, req: Request<ReqBody>) -> Self::Future {
        // 1. 生成缓存 key（基于请求 URI）
        let cache_key = format!("cache:{}", req.uri().path());
        
        // 2. 检查缓存
        // 3. 如果缓存命中，直接返回
        // 4. 如果未命中，调用 inner service，然后缓存响应
    }
}
```

#### 优点

- ✅ **统一处理**：Middleware 拦截，业务逻辑与缓存分离
- ✅ **统一配置**：可以在 Router 级别统一配置缓存策略
- ✅ **零成本抽象**：编译时优化

#### 缺点

- ⚠️ **缓存 key 难以与 handler 绑定**：需要额外配置或约定
- ⚠️ **灵活性较低**：难以针对不同 handler 使用不同的缓存 key 和 TTL
- ⚠️ **实现复杂度中等**：需要实现 Tower Service trait

#### 适用场景

- 需要统一缓存策略的项目
- 缓存策略相对固定的场景
- 团队熟悉 Tower 中间件

---

### 方案 3：宏简化（当前方案优化版）⭐⭐⭐⭐

#### 实现思路

使用声明宏简化 `get_cached_or_render` 的调用，减少代码冗余。

#### 代码示例

```rust
// 定义宏
#[macro_export]
macro_rules! cached {
    ($cache:expr, $key:expr, $render:expr) => {{
        match crate::services::get_cached_or_render(
            $cache,
            $key,
            move || async move { $render.await },
            None,
        ).await {
            Ok(bytes) => axum::response::Html(bytes).into_response(),
            Err(e) => crate::views::handle_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                e
            ).await,
        }
    }};
    
    // 支持自定义 TTL
    ($cache:expr, $key:expr, $render:expr, $ttl:expr) => {{
        let config = crate::services::CacheConfig {
            dev_ttl: $ttl,
            prod_ttl: $ttl,
        };
        match crate::services::get_cached_or_render(
            $cache,
            $key,
            move || async move { $render.await },
            Some(config),
        ).await {
            Ok(bytes) => axum::response::Html(bytes).into_response(),
            Err(e) => crate::views::handle_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                e
            ).await,
        }
    }};
}

// Controller 中使用
async fn index(
    Extension(cache): Extension<CacheExtension>,
    Extension(manifest): Extension<ManifestExtension>,
) -> impl IntoResponse {
    cached!(
        &*cache,
        "home:index",
        render_home_index(manifest.clone())
    )
}

// 动态 key
async fn blog_detail(
    Path(vid): Path<String>,
    Extension(cache): Extension<CacheExtension>,
    Extension(manifest): Extension<ManifestExtension>,
) -> impl IntoResponse {
    cached!(
        &*cache,
        format!("blog:detail:{}", vid),
        render_blog_detail(vid, manifest.clone())
    )
}

// 自定义 TTL
async fn blog_list(
    Extension(cache): Extension<CacheExtension>,
    Extension(manifest): Extension<ManifestExtension>,
) -> impl IntoResponse {
    cached!(
        &*cache,
        "blog:list",
        render_blog_list(None, manifest.clone()),
        7200  // TTL: 2 hours
    )
}
```

#### 实现位置

```rust
// src/services/cache.rs 或 src/macros.rs
#[macro_export]
macro_rules! cached {
    // ... 如上所示
}
```

#### 优点

- ✅ **作用在 Controller 层**：符合最佳实践
- ✅ **实现简单**：只需声明宏，无需 proc-macro
- ✅ **灵活**：支持动态 key，完全灵活
- ✅ **零成本抽象**：宏展开，无运行时开销
- ✅ **与当前架构兼容**：基于现有的 `get_cached_or_render` 函数

#### 缺点

- ⚠️ **不够声明式**：需要显式调用宏（不如属性宏优雅）
- ⚠️ **代码仍有一定冗余**：虽然简化了，但仍需要传递 cache 和 key

#### 适用场景

- 快速实现和迭代
- 需要最大灵活性
- 不想引入 proc-macro 依赖

---

## 方案对比总结

| 方案 | 类型安全 | 零成本 | 简洁度 | 灵活性 | 维护成本 | 实现复杂度 | 推荐度 |
|------|---------|--------|--------|--------|---------|-----------|--------|
| **属性宏** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Tower Middleware** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **宏简化** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |

---

## 推荐方案

### 短期推荐：宏简化（方案 3）

**理由**：
1. ✅ **快速实现**：只需添加声明宏，无需额外依赖
2. ✅ **作用在 Controller 层**：符合最佳实践
3. ✅ **完全灵活**：支持动态 key，满足所有场景
4. ✅ **零成本抽象**：编译时展开
5. ✅ **与当前架构兼容**：基于现有实现，改动最小

**实施步骤**：
1. 在 `src/services/cache.rs` 或新建 `src/macros.rs` 中添加 `cached!` 宏
2. 更新所有 Controller 使用宏简化调用
3. 测试验证

### 长期推荐：属性宏（方案 1）

**理由**：
1. ✅ **声明式**：代码最简洁优雅
2. ✅ **完全零成本**：编译时展开
3. ✅ **类型安全**：编译期检查
4. ✅ **高度灵活**：支持动态 key

**实施步骤**：
1. 创建 `oic-web-macros` proc-macro crate
2. 实现 `#[cacheable]` 属性宏
3. 更新所有 Controller 使用属性宏
4. 测试验证

### 特殊场景推荐：Tower Middleware（方案 2）

**适用场景**：
- 需要统一缓存策略
- 缓存策略相对固定
- 团队熟悉 Tower 中间件

---

## 当前实现（已优化）

### 当前代码（`src/services/cache.rs`）

```rust
/// 获取缓存或渲染新内容（统一实现）
/// 
/// 使用 `Bytes` 数据类型进行缓存和返回，零拷贝，性能最优。
pub async fn get_cached_or_render<F, Fut>(
    cache: &Cache,
    cache_key: &str,
    render_fn: F,
    config: Option<CacheConfig>,
) -> Result<Bytes>  // ✅ 返回 Bytes
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Bytes, anyhow::Error>>,
{
    // 先检查缓存（使用底层 get API，返回 Bytes）
    if let Ok(Some(bytes)) = cache.get(cache_key).await {
        // ✅ 直接返回 Bytes，零拷贝
        return Ok(bytes);
    }

    // 缓存未命中，调用渲染函数
    let bytes: Bytes = render_fn().await?;

    // 确定 TTL 并缓存
    let config = config.unwrap_or_default();
    let ttl_seconds = if cfg!(debug_assertions) {
        config.dev_ttl
    } else {
        config.prod_ttl
    };

    if let Err(e) = cache.set_with_ttl(
        cache_key.to_string(),
        bytes.clone(),
        "text/html".to_string(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache: {}", e);
    }

    // ✅ 直接返回 Bytes，零拷贝
    Ok(bytes)
}
```

### Controller 层使用（当前）

```rust
// controllers/home.rs
async fn index(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let manifest_clone = manifest.clone();
    
    match get_cached_or_render(
        &*cache,
        "home:index",
        move || {
            let manifest = manifest_clone.clone();
            async move {
                render_home_index(manifest).await
            }
        },
        None,
    ).await {
        Ok(bytes) => Html(bytes).into_response(),
        Err(e) => handle_error(StatusCode::INTERNAL_SERVER_ERROR, e).await,
    }
}
```

### View 层优化（已实现）

View 层函数直接返回 `Bytes`，使用 `RenderBytes` trait 简化模板渲染：

```rust
// src/models/tpl.rs
pub trait RenderBytes: Template {
    fn render_bytes(&self) -> Result<Bytes> {
        let mut buffer = Vec::new();
        Template::write_into(self, &mut buffer)
            .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
        Ok(Bytes::from(buffer))
    }
}

impl<T: Template> RenderBytes for T {}

// src/views/home.rs
pub async fn render_home_index(
    manifest: Arc<HashMap<String, ManifestChunk>>
) -> Result<Bytes> {
    // ... 数据获取和模板构建 ...
    let template = HomeTemplate { ... };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}
```

**优势**：
- ✅ 避免 String 中间转换，直接渲染到 `Vec<u8>` 然后转为 `Bytes`
- ✅ 代码简洁：只需调用 `template.render_bytes()`
- ✅ 零成本抽象：编译时展开

---

## 性能对比

| 操作 | 优化前 | 优化后 |
|------|--------|--------|
| **缓存命中** | `Bytes` → `Vec<u8>` → `String` | `Bytes`（零拷贝） |
| **缓存未命中** | `Bytes` → `Vec<u8>` → `String` | `Bytes`（零拷贝） |
| **模板渲染** | `String` → `Bytes` | `Vec<u8>` → `Bytes`（更高效） |
| **内存分配** | 2 次（`to_vec` + `String`） | 1 次（`Vec<u8>`） |
| **UTF-8 验证** | 每次返回都验证 | 不需要（直接 bytes） |

### 性能收益

假设一个典型的 HTML 页面大小为 **50KB**：

- **模板渲染优化**：使用 `write_into` 直接写入 `Vec<u8>`，避免 String 分配
- **零拷贝返回**：`Bytes` 使用引用计数，Clone 零成本
- **整体性能提升**：约 **10-20%**（取决于页面大小）

---

## 错误处理方案

### 问题

当前实现中，`get_cached_or_render` 出错时返回纯文本错误信息，暴露技术细节给用户。

### 解决方案：友好的错误页面

#### 1. 创建错误页面模板

**文件**：`templates/error.html`

```html
{% extends "base.html" %}

{% block title %}页面加载失败 - 灵犀纪{% endblock %}

{% block content %}
<div class="error-container" style="max-width: 600px; margin: 100px auto; text-align: center; padding: 20px;">
    <h1 style="font-size: 48px; margin-bottom: 20px; color: #333;">😔</h1>
    <h2 style="font-size: 24px; margin-bottom: 16px; color: #333;">抱歉，页面加载失败</h2>
    <p style="font-size: 16px; color: #666; margin-bottom: 32px; line-height: 1.6;">
        我们正在努力修复这个问题，请稍后再试。
    </p>
    <div style="display: flex; gap: 16px; justify-content: center;">
        <a href="/" style="display: inline-block; padding: 12px 24px; background: #007bff; color: white; text-decoration: none; border-radius: 4px;">
            返回首页
        </a>
        <button onclick="window.location.reload()" style="padding: 12px 24px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;">
            刷新页面
        </button>
    </div>
</div>
{% endblock %}
```

#### 2. 创建错误处理模块

**文件**：`src/views/error.rs`

```rust
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use crate::models::RenderBytes;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    #[allow(dead_code)]
    pub status_code: u16,
    pub title: String,
    pub message: String,
}

impl ErrorTemplate {
    pub fn new(status_code: StatusCode) -> Self {
        let (title, message) = match status_code {
            StatusCode::NOT_FOUND => (
                "页面未找到".to_string(),
                "抱歉，您访问的页面不存在。".to_string(),
            ),
            StatusCode::INTERNAL_SERVER_ERROR => (
                "服务器错误".to_string(),
                "我们正在努力修复这个问题，请稍后再试。".to_string(),
            ),
            _ => (
                "页面加载失败".to_string(),
                "抱歉，页面加载失败，请稍后再试。".to_string(),
            ),
        };

        Self {
            status_code: status_code.as_u16(),
            title,
            message,
        }
    }
}

/// 统一的错误处理函数
pub async fn handle_error(status_code: StatusCode, error: anyhow::Error) -> Response {
    eprintln!("Application error ({}): {}", status_code, error);
    
    let template = ErrorTemplate::new(status_code);
    match template.render_bytes() {
        Ok(bytes) => (status_code, Html(bytes)).into_response(),
        Err(_) => {
            // 降级：如果模板渲染失败，返回简单 HTML
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <title>错误</title>
</head>
<body>
    <h1>😔</h1>
    <h2>页面加载失败</h2>
    <p>我们正在努力修复这个问题，请稍后再试。</p>
    <a href="/">返回首页</a>
</body>
</html>"#
            );
            (status_code, Html(html)).into_response()
        }
    }
}
```

#### 3. 在 Controller 中使用

```rust
use crate::views::handle_error;

match get_cached_or_render(...).await {
    Ok(bytes) => Html(bytes).into_response(),
    Err(e) => handle_error(StatusCode::INTERNAL_SERVER_ERROR, e).await,
}
```

---

## 相关文档

- [Cache Askama 集成方案](../../../docs/cache-askama.md)
- [Cache Bytes 优化](../../../docs/cache-bytes.md)
- [oic-cache Bytes 迁移总结](../../oic-cache/BYTES_MIGRATION.md)

---

## 状态

- [x] 当前实现已优化（返回 Bytes，View 层使用 RenderBytes trait）
- [ ] 方案 1：属性宏（长期推荐）
- [ ] 方案 2：Tower Middleware（可选）
- [ ] 方案 3：宏简化（短期推荐）

**创建时间**：2025-01-XX  
**最后更新**：2025-01-XX  
**优先级**：中（性能优化，非阻塞）
