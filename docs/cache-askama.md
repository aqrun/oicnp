# Cache Crate 架构设计问题

##  问题

我的cache独立的crate， web项目也是独立的crate. 那 
```
cache.get_or_render_async(
    "page:home",
    || async { db.get_posts().await },  // 异步获取数据
    |posts| HomeTemplate { posts }       // 同步构建模板
)
 ```
这个缓存处理是在 cache里实现还是在 web项目里实现呢

## 问题分析

你有两个独立 crate：
- `my_cache` - 缓存库
- `my_web` - Web 应用（依赖 my_cache）

关键问题：`get_or_render_async` 应该在哪里实现？

---

## 方案对比

### 方案 1：在 Cache Crate 中实现（❌ 不推荐）

#### 问题 1：依赖倒置

```
my_cache (缓存库)
    ↓ 需要依赖
askama (模板引擎)
```

**问题**：
- ❌ 缓存库不应该知道模板的存在
- ❌ 违反依赖倒置原则
- ❌ 缓存库变得不通用（只能用于模板）

#### 问题 2：通用性丧失

```rust
// Cache crate 变得特化
impl CacheManager {
    pub async fn get_or_render_async<T: Template>(...) {
        // 只能缓存 Template，不能缓存其他类型
    }
}
```

**问题**：
- ❌ 无法缓存 JSON、Protobuf 等其他格式
- ❌ 与 askama 强耦合

---

### 方案 2：在 Web Crate 中实现（✅ 推荐）

#### 架构清晰

```
my_cache (通用缓存库)
    ↓
my_web (Web 应用)
    ├── 依赖 my_cache
    └── 依赖 askama
    
my_web 实现：
    - 模板渲染逻辑
    - 缓存 + 模板的组合逻辑
```

**优点**：
- ✅ 缓存库保持通用
- ✅ 依赖方向正确
- ✅ 各司其职

---

## 推荐架构设计

### Cache Crate：提供通用能力

```rust
// my_cache/src/lib.rs

use bytes::Bytes;

/// 通用缓存管理器
pub struct CacheManager {
    cache: Arc<moka::future::Cache<String, Bytes>>,
}

impl CacheManager {
    pub fn new() -> Self { /* ... */ }
    
    /// 基础方法：获取
    pub async fn get(&self, key: &str) -> Option<Bytes> {
        self.cache.get(key).await
    }
    
    /// 基础方法：设置
    pub async fn set(&self, key: String, value: Bytes) {
        self.cache.insert(key, value).await;
    }
    
    /// 基础方法：删除
    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }
    
    /// 通用方法：获取或计算（同步）
    pub async fn get_or_compute<F>(
        &self,
        key: impl Into<String>,
        compute_fn: F,
    ) -> Bytes
    where
        F: FnOnce() -> Bytes,
    {
        let key = key.into();
        
        if let Some(cached) = self.get(&key).await {
            return cached;
        }
        
        let value = compute_fn();
        self.set(key, value.clone()).await;
        value
    }
    
    /// 通用方法：获取或异步计算
    pub async fn get_or_compute_async<F, Fut>(
        &self,
        key: impl Into<String>,
        compute_fn: F,
    ) -> Bytes
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Bytes>,
    {
        let key = key.into();
        
        if let Some(cached) = self.get(&key).await {
            return cached;
        }
        
        let value = compute_fn().await;
        self.set(key, value.clone()).await;
        value
    }
}
```

**特点**：
- ✅ 完全不知道 askama
- ✅ 只处理 `String` → `Bytes` 的映射
- ✅ 可用于任何场景（JSON、HTML、Protobuf 等）

---

### Web Crate：实现模板相关逻辑

```rust
// my_web/src/cache_ext.rs

use my_cache::CacheManager;
use askama::Template;
use bytes::Bytes;
use crate::error::AppError;

/// 为 CacheManager 扩展模板相关功能
pub trait CacheTemplateExt {
    /// 获取或渲染（同步）
    async fn get_or_render_sync<T, F>(
        &self,
        key: impl Into<String>,
        template_fn: F,
    ) -> Result<Bytes, AppError>
    where
        T: Template,
        F: FnOnce() -> T;
    
    /// 获取或渲染（异步数据 + 同步模板）
    async fn get_or_render_async<T, D, F, Fut>(
        &self,
        key: impl Into<String>,
        data_fn: F,
        template_fn: impl FnOnce(D) -> T,
    ) -> Result<Bytes, AppError>
    where
        T: Template,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<D, AppError>>;
}

/// 实现扩展 trait
impl CacheTemplateExt for CacheManager {
    async fn get_or_render_sync<T, F>(
        &self,
        key: impl Into<String>,
        template_fn: F,
    ) -> Result<Bytes, AppError>
    where
        T: Template,
        F: FnOnce() -> T,
    {
        let bytes = self.get_or_compute(key, || {
            let template = template_fn();
            render_to_bytes(&template).unwrap()
        }).await;
        
        Ok(bytes)
    }
    
    async fn get_or_render_async<T, D, F, Fut>(
        &self,
        key: impl Into<String>,
        data_fn: F,
        template_fn: impl FnOnce(D) -> T,
    ) -> Result<Bytes, AppError>
    where
        T: Template,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<D, AppError>>,
    {
        let key = key.into();
        
        // 先查缓存
        if let Some(cached) = self.get(&key).await {
            return Ok(cached);
        }
        
        // 异步获取数据
        let data = data_fn().await?;
        
        // 同步渲染模板
        let template = template_fn(data);
        let bytes = render_to_bytes(&template)?;
        
        // 缓存
        self.set(key, bytes.clone()).await;
        
        Ok(bytes)
    }
}

/// 辅助函数：渲染模板为 Bytes
fn render_to_bytes<T: Template>(template: &T) -> Result<Bytes, askama::Error> {
    let mut buf = Vec::with_capacity(4096);
    template.render_into(&mut buf)?;
    Ok(Bytes::from(buf))
}
```

---

### Web Crate：Handler 使用

```rust
// my_web/src/handlers/home.rs

use axum::extract::State;
use my_cache::CacheManager;
use crate::cache_ext::CacheTemplateExt;  // 导入扩展 trait
use crate::templates::HomeTemplate;

pub async fn home_handler(
    State(cache): State<CacheManager>,
    State(db): State<Database>,
) -> Result<HtmlResponse, AppError> {
    // 使用扩展方法
    let bytes = cache.get_or_render_async(
        "page:home",
        || async { 
            let posts = db.get_posts().await?;
            Ok(posts)
        },
        |posts| HomeTemplate { posts }
    ).await?;
    
    Ok(HtmlResponse(bytes))
}
```

---

## 完整架构图

```
┌─────────────────────────────────────────┐
│        my_cache (通用缓存库)              │
│  - CacheManager                         │
│  - get() / set() / invalidate()         │
│  - get_or_compute()                     │
│  - get_or_compute_async()               │
│                                         │
│  依赖：bytes, moka                       │
│  不依赖：askama ✅                       │
└─────────────────────────────────────────┘
                ↑
                │ 依赖
                │
┌─────────────────────────────────────────┐
│         my_web (Web 应用)                │
│                                         │
│  cache_ext.rs:                          │
│    - trait CacheTemplateExt             │
│    - impl CacheTemplateExt              │
│    - get_or_render_sync()               │
│    - get_or_render_async()              │
│    - render_to_bytes()                  │
│                                         │
│  handlers/*.rs:                         │
│    - 使用 CacheTemplateExt 扩展方法      │
│                                         │
│  依赖：my_cache, askama, bytes          │
└─────────────────────────────────────────┘
```

---

## 为什么这样设计？

### 1. 职责分离

| Crate | 职责 |
|-------|------|
| **my_cache** | 通用缓存能力（Key-Value 存储） |
| **my_web** | 业务逻辑（模板渲染 + 缓存组合） |

### 2. 依赖方向正确

```
通用库 ← 业务应用  ✅

而不是：
通用库 → 业务库   ❌
```

### 3. 可复用性

**my_cache 可用于**：
- 缓存 HTML（你的场景）
- 缓存 JSON API 响应
- 缓存 Protobuf 数据
- 缓存图片
- 任何 `Bytes` 数据

**示例：在其他项目中使用**

```rust
// 项目 A：缓存 HTML
impl CacheTemplateExt for CacheManager { /* askama */ }

// 项目 B：缓存 JSON
impl CacheJsonExt for CacheManager {
    async fn get_or_serialize<T: Serialize>(...) { /* serde */ }
}

// 项目 C：缓存图片
impl CacheImageExt for CacheManager {
    async fn get_or_encode<T: Image>(...) { /* image */ }
}
```

### 4. 易于测试

```rust
// 测试 Cache 不需要 askama
#[cfg(test)]
mod tests {
    use my_cache::CacheManager;
    
    #[tokio::test]
    async fn test_cache() {
        let cache = CacheManager::new();
        cache.set("key".into(), Bytes::from("value")).await;
        assert_eq!(cache.get("key").await, Some(Bytes::from("value")));
    }
}
```

---

## 其他方案（可选）

### 方案 3：中间 Adapter Crate（过度设计）

```
my_cache (通用缓存)
    ↓
my_cache_askama (Askama 适配器)
    ↓
my_web (应用)
```

**何时使用**：
- 多个 Web 项目共享 Askama 缓存逻辑
- 需要发布为独立的 askama-cache 库

**大多数情况不需要**

---

### 方案 4：泛型 Trait（过于复杂）

```rust
// my_cache/src/lib.rs
pub trait Cacheable {
    fn to_bytes(&self) -> Bytes;
    fn cache_key(&self) -> String;
}

impl CacheManager {
    pub async fn get_or_compute<T: Cacheable>(...) { }
}

// my_web 中实现
impl Cacheable for HomeTemplate {
    fn to_bytes(&self) -> Bytes {
        render_to_bytes(self).unwrap()
    }
}
```

**问题**：
- 过度抽象
- Cache crate 依然知道了 Cacheable 的存在
- 不如扩展 trait 简单

---

## 最终推荐

### ✅ 推荐方案：Extension Trait Pattern

**实现位置**：在 **Web Crate** 中

**文件结构**：
```
my_web/
├── src/
│   ├── cache_ext.rs          # CacheTemplateExt trait
│   ├── handlers/
│   │   └── home.rs           # 使用 cache.get_or_render_*
│   └── main.rs
```

**核心代码**：
```rust
// my_web/src/cache_ext.rs
pub trait CacheTemplateExt {
    async fn get_or_render_sync(...) -> Result<Bytes>;
    async fn get_or_render_async(...) -> Result<Bytes>;
}

impl CacheTemplateExt for my_cache::CacheManager {
    // 实现
}
```

**使用**：
```rust
use crate::cache_ext::CacheTemplateExt;

// 自动拥有扩展方法
cache.get_or_render_sync(...).await?;
```

---

## 总结

| 方面 | Cache Crate | Web Crate |
|------|------------|-----------|
| **职责** | 通用缓存 | 业务逻辑 |
| **依赖** | bytes, moka | my_cache, askama |
| **方法** | get, set, get_or_compute | get_or_render_* |
| **知道模板吗？** | ❌ 不知道 | ✅ 知道 |
| **可复用性** | ✅ 高 | ❌ 特定项目 |

**最佳实践**：
1. Cache Crate：提供通用的 `get_or_compute`
2. Web Crate：实现 Extension Trait 添加 `get_or_render_*`
3. Handler：使用扩展方法

这样既保持了缓存库的通用性，又提供了方便的模板缓存能力。