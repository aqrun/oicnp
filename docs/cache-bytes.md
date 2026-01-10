# Rust + Axum + Askama 缓存优化方案

## 一、数据类型选择

### Vec<u8> vs String vs Bytes 对比

| 类型 | 特点 | Clone 成本 | 推荐度 |
|------|------|-----------|--------|
| **String** | UTF-8 保证，类型安全 | 复制所有数据 O(n) | ⭐⭐⭐ |
| **Vec<u8>** | 原始字节，灵活 | 复制所有数据 O(n) | ⭐⭐⭐ |
| **Bytes** | 引用计数，零拷贝 Clone | 只增加计数器 O(1) | ⭐⭐⭐⭐⭐ |

### 最终推荐：Bytes

**理由**：
- ✅ Clone 零拷贝（只增加引用计数）
- ✅ Askama 支持渲染到 `Vec<u8>`，可零成本转为 Bytes
- ✅ Axum 原生支持 Bytes 作为响应
- ✅ 网络库标准类型（Tokio 生态）

**性能差异**：
```
Vec<u8> Clone 1MB：~0.5ms（复制数据）
Bytes Clone 1MB：  ~0.0001ms（只增加计数器）
快 5000 倍！
```

**三者转换**：
```
String → Vec<u8>:  零开销（into_bytes()）
Vec<u8> → Bytes:   零开销（Bytes::from()）
Vec<u8> → String:  需要 UTF-8 验证（~15ns）
```

---

## 二、模板缓存抽象方案

### 方案 1：Trait 扩展 ⭐⭐⭐⭐

**思路**：为模板定义统一的缓存接口

**核心 API**：
```rust
trait CacheableTemplate {
    fn cache_key(&self) -> String;
    fn cache_ttl(&self) -> u64;
    fn cacheable(&self) -> bool;
    async fn get_or_render(&self, cache: &Cache) -> Bytes;
}
```

**使用示例**：
```rust
// 实现 trait
impl CacheableTemplate for HomeTemplate {
    fn cache_key(&self) -> String {
        "page:home".to_string()
    }
}

// Handler 中使用
let bytes = template.get_or_render(&cache).await?;
```

**优点**：类型安全、每个模板控制自己的缓存策略
**缺点**：需要为每个模板实现 trait、需要 async_trait

---

### 方案 2：声明宏 ⭐⭐⭐⭐

**思路**：用宏自动实现 trait

**使用示例**：
```rust
// 声明缓存策略
cacheable_template!(
    HomeTemplate,
    key = "page:home",
    ttl = 1800
);

cacheable_template!(
    PostTemplate,
    key = |t| format!("post:{}", t.post.id),
    ttl = 7200
);

// Handler 中使用（同方案 1）
let bytes = template.get_or_render(&cache).await?;
```

**优点**：声明式、减少重复代码
**缺点**：宏语法不直观、调试困难

---

### 方案 3：过程宏（Derive） ⭐⭐⭐⭐⭐

**思路**：使用 derive 宏自动生成实现

**使用示例**：
```rust
#[derive(Template, Cacheable)]
#[cache(key = "page:home", ttl = 1800)]
struct HomeTemplate { ... }

#[derive(Template, Cacheable)]
#[cache(key_fn = "format!(\"post:{}\", self.post.id)", ttl = 7200)]
struct PostTemplate { ... }

// 不缓存
#[derive(Template, Cacheable)]
#[cache(disabled)]
struct AdminTemplate { ... }
```

**优点**：最简洁、声明式
**缺点**：需要单独的 proc-macro crate、编译时间增加

---

### 方案 4：Builder 模式 ⭐⭐⭐⭐⭐

**思路**：链式调用配置缓存

**使用示例**：
```rust
// 基础用法
let bytes = HomeTemplate { posts }
    .cached()
    .key("page:home")
    .ttl(1800)
    .render(&cache)
    .await?;

// 动态配置
let bytes = PostTemplate { post }
    .cached()
    .key(format!("post:{}", id))
    .ttl(7200)
    .render(&cache)
    .await?;

// 禁用缓存
let bytes = AdminTemplate { data }
    .cached()
    .no_cache()
    .render(&cache)
    .await?;
```

**优点**：灵活、每次调用可不同配置、不需要宏、易于理解
**缺点**：代码略长

---

### 方案 5：全局函数 ⭐⭐⭐

**思路**：提供全局缓存函数

**使用示例**：
```rust
// 简单场景
let bytes = render_cached(
    &cache,
    "page:home",
    || HomeTemplate { posts }
).await?;

// 动态键
let bytes = render_cached(
    &cache,
    format!("post:{}", id),
    || PostTemplate { post }
).await?;
```

**优点**：实现简单、零开销
**缺点**：不够优雅、缺少类型约束

---

### 方案 6：混合方案（推荐） ⭐⭐⭐⭐⭐

**思路**：Trait 默认实现 + Builder 灵活配置

**使用示例**：
```rust
// 简单场景（使用默认配置）
let bytes = template.get_or_render(&cache).await?;

// 复杂场景（动态配置）
let bytes = template
    .with_cache()
    .key(format!("post:{}", id))
    .ttl(7200)
    .render(&cache)
    .await?;

// 条件缓存
let bytes = if is_own_profile {
    template.with_cache().no_cache().render(&cache).await?
} else {
    template.get_or_render(&cache).await?
};
```

**优点**：兼顾简洁和灵活
**适合**：大多数实际项目

---

## 三、终极方案：分离数据获取和渲染 ⭐⭐⭐⭐⭐

### 核心问题

前面所有方案的致命问题：**Rust 闭包无法捕获 async 上下文**

```rust
// ❌ 这样不行
cache.get_or_render("key", || {
    let data = db.query().await;  // 错误：不能在闭包中 await
    Template { data }
})
```

### 解决方案

**关键洞察**：分离「数据获取（异步）」和「模板渲染（同步）」

### 三个核心方法

#### 方法 1：同步渲染

```rust
cache.get_or_render_sync(
    "page:home",
    || HomeTemplate { posts }
)
```

**适用**：数据已在 Handler 中获取

#### 方法 2：异步数据 + 同步渲染（最实用）

```rust
cache.get_or_render_async(
    "page:home",
    || async { db.get_posts().await },  // 异步获取数据
    |posts| HomeTemplate { posts }       // 同步构建模板
)
```

**适用**：数据获取逻辑简单，追求代码简洁

#### 方法 3：直接渲染（不缓存）

```rust
CacheManager::render(AdminTemplate { stats })
```

**适用**：动态内容、个性化页面

---

## 四、三种使用模式

### 模式 1：数据在 Handler 中获取（最灵活）

```rust
async fn handler(cache, db) -> Result<Response> {
    // 1. 异步获取数据
    let posts = db.get_posts().await?;
    let featured = db.get_featured().await.ok();
    
    // 2. 同步缓存渲染
    let bytes = cache.get_or_render_sync(
        "page:home",
        || HomeTemplate { posts, featured }
    ).await?;
    
    Ok(Response(bytes))
}
```

**优点**：控制流清晰、易调试、最灵活

### 模式 2：使用 get_or_render_async（更简洁）

```rust
async fn handler(cache, db) -> Result<Response> {
    let bytes = cache.get_or_render_async(
        "page:home",
        || async {
            let posts = db.get_posts().await?;
            let featured = db.get_featured().await.ok();
            Ok((posts, featured))
        },
        |(posts, featured)| HomeTemplate { posts, featured }
    ).await?;
    
    Ok(Response(bytes))
}
```

**优点**：代码紧凑、逻辑内聚

### 模式 3：条件缓存

```rust
async fn profile_handler(cache, db, user_id, current_user) -> Result<Response> {
    let user = db.get_user(user_id).await?;
    let template = ProfileTemplate { user };
    
    let bytes = if current_user.id == user_id {
        // 自己的页面不缓存
        CacheManager::render(template)?
    } else {
        // 别人的页面缓存
        cache.get_or_render_sync(
            format!("profile:{}", user_id),
            || template
        ).await?
    };
    
    Ok(Response(bytes))
}
```

**优点**：精细控制

---

## 五、方案推荐

### 按项目规模

| 规模 | 推荐方案 |
|------|---------|
| **个人项目/MVP** | 全局函数方案 |
| **小型项目** | Builder 模式 |
| **中型项目** | 混合方案 |
| **大型项目** | 分离数据渲染 |

### 按复杂度

| 场景 | 推荐方法 |
|------|---------|
| **数据已获取** | get_or_render_sync |
| **简单数据获取** | get_or_render_async |
| **复杂逻辑** | 模式 1（数据在 Handler 获取） |
| **不缓存** | render |

### 最佳实践

**推荐组合**：分离数据渲染 + Builder 模式

```rust
// 简单场景
cache.get_or_render_sync(key, || template).await

// 复杂场景
cache.get_or_render_async(key, || async {...}, |data| template).await

// 不缓存
CacheManager::render(template)
```

---

## 总结

1. **数据类型**：使用 `Bytes`（零拷贝）
2. **核心方案**：分离数据获取和渲染（解决 async 闭包问题）
3. **使用模式**：根据场景选择三种方法
4. **最佳实践**：灵活组合，简单场景用简单方法，复杂场景用模式 1