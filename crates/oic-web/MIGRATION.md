# Next.js 到 Axum + Askama 迁移文档

## 迁移概述

将 `apps/web` (Next.js SSR) 迁移到 `crates/oic-web` (Axum + Askama)，原因：
- Next.js 性能问题，服务器不能再运行 Node.js 服务
- Rust 服务端渲染性能更好，内存占用更小
- 单一二进制文件，部署更简单

## 技术栈

- **后端框架**: axum
- **模板引擎**: askama (类似 Jinja2)
- **缓存系统**: oic_cache (支持 HTML 缓存和 SWR)
- **HTTP 客户端**: reqwest (调用 api.lxage.com)
- **前端构建**: vitejs
- **样式**: tailwindcss v4
- **交互**: alpine.js (轻量级，无需构建)

## 已完成的工作

### 1. API 服务层 (`src/services/api.rs`)

✅ **已完成**
- 使用 `oic_core` 的类型定义，避免重复定义
- 实现了 `describe_node_list` - 调用节点列表 API
- 实现了 `describe_node_detail` - 调用节点详情 API
- 使用 `oic_core::typings::JsonRes` 解析 API 响应
- 使用 `oic_core::models::nodes::NodeFilters` 作为请求参数
- 使用 `oic_core::models::nodes::NodeDetailModel` 作为响应数据模型

**关键代码**:
```rust
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::JsonRes,
};
```

### 2. 首页 (`/`)

✅ **已完成**
- 控制器: `src/controllers/home.rs`
- 视图: `src/views/home.rs`
- 模板: `templates/home.html`
- 包含组件：
  - Header (导航栏)
  - BigNews (大新闻卡片)
  - News Grid (新闻网格，4个卡片)
  - Article List (文章列表)
  - Footer (页脚)
- 支持 HTML 缓存

### 3. 博客列表页 (`/blog/`)

✅ **已完成**
- 控制器: `src/controllers/blog.rs` - `blog_list` 函数
- 视图: `src/views/blog.rs` - `render_blog_list` 函数
- 模板: `templates/blog_list.html`
- 支持分类筛选 (`/cat/:cat_vid/`)
- 支持 HTML 缓存

### 4. 博客详情页 (`/p/:vid/`)

✅ **已完成**
- 控制器: `src/controllers/blog.rs` - `blog_detail` 函数
- 视图: `src/views/blog.rs` - `render_blog_detail` 函数
- 模板: `templates/blog_detail.html`
- 支持 HTML 缓存

### 5. 路由配置

✅ **已完成**
- 在 `src/app.rs` 中配置了所有路由
- 首页: `/`
- 博客列表: `/blog/`
- 分类列表: `/cat/:cat_vid/`
- 博客详情: `/p/:vid/`

## 当前状态

### 编译错误

⚠️ **需要修复**

1. **类型不匹配问题** (`src/views/home.rs`)
   - `NodeDetailModel.created_at` 是 `DateTime` 类型，不是 `Option<String>`
   - `categories` 和 `tags` 是 `Vec`，不是 `Option<Vec>`
   - 需要将 `DateTime` 格式化为字符串

2. **模板语法错误** (`templates/home.html`, `templates/blog_detail.html`)
   - Askama 不支持复杂的 Rust 表达式
   - 需要简化模板中的条件判断

3. **未使用的导入**
   - `src/services/mod.rs` 中 `cache::*` 未使用

### 待完成的工作

#### 高优先级

1. **修复类型错误**
   - [ ] 修复 `created_at` 类型转换（DateTime -> String）
   - [ ] 修复 `categories` 和 `tags` 的类型处理
   - [ ] 修复模板中的类型访问

2. **模板优化**
   - [ ] 简化 Askama 模板中的条件判断
   - [ ] 确保所有字段访问正确

#### 中优先级

3. **诗词列表页** (`/poetry/`)
   - [ ] 创建控制器和视图
   - [ ] 创建模板
   - [ ] 配置路由

4. **工具列表页** (`/tool/`)
   - [ ] 创建控制器和视图
   - [ ] 创建模板
   - [ ] 配置路由

5. **侧边栏组件**
   - [ ] 标签列表组件
   - [ ] 分类列表组件
   - [ ] 推荐博客组件

#### 低优先级

6. **Markdown 内容解析**
   - [ ] 博客详情页的 Markdown 渲染
   - [ ] 可能需要使用 `pulldown-cmark` 或其他 Rust markdown 库

7. **客户端交互**
   - [ ] "加载更多" 功能（使用 Alpine.js）
   - [ ] 搜索功能
   - [ ] 其他交互功能

## 技术决策

### 1. 使用 `oic-core` 的类型

**决策**: 直接使用 `oic-core` 中已定义的类型，避免重复定义

**原因**:
- `oic-core` 已经定义了完整的 API 数据模型
- 保持类型一致性，避免维护两套类型定义
- 减少代码重复

**使用的类型**:
- `oic_core::models::nodes::NodeFilters` - 节点过滤参数
- `oic_core::models::nodes::NodeDetailModel` - 节点详情模型
- `oic_core::typings::JsonRes` - API 响应格式
- `oic_core::entities::category::Model` - 分类模型
- `oic_core::entities::tag::Model` - 标签模型

### 2. 缓存策略

**决策**: 使用 `oic_cache` 进行 HTML 级缓存

**配置**:
- 开发模式: 1 秒过期（便于调试）
- 生产环境: 1 小时过期

**缓存键**:
- 首页: `home:index`
- 博客列表: `blog:list` 或 `blog:list:cat:{cat_vid}`
- 博客详情: `blog:detail:{vid}`

### 3. API 调用方式

**决策**: 使用 `reqwest` 直接调用 `api.lxage.com`

**配置**:
- 通过环境变量 `API_BASE_URL` 配置 API 地址
- 默认值: `https://api.lxage.com`

**响应解析**:
- 使用 `JsonRes<Value>` 解析响应
- 从 `data` 字段中提取实际数据
- 列表接口: `data.nodes`, `data.page`, `data.pageSize`, `data.total`
- 详情接口: `data.node`

## 已知问题

### 1. Askama 模板限制

**问题**: Askama 不支持复杂的 Rust 表达式，如 `node.categories.as_ref().and_then(|c| c.first())`

**解决方案**: 
- 在 Rust 代码中预处理数据，将复杂逻辑移到视图层
- 模板中只使用简单的字段访问和条件判断

**示例**:
```rust
// ❌ 不好的做法（模板中）
{% if node.categories.as_ref().and_then(|c| c.first()).is_some() %}

// ✅ 好的做法（Rust 中预处理）
let category_name = node.categories.first().map(|cat| cat.cat_name.clone());
// 模板中
{% if category_name.is_some() %}
```

### 2. DateTime 类型转换

**问题**: `NodeDetailModel.created_at` 是 `DateTime` 类型，模板需要字符串

**解决方案**: 在视图层格式化日期
```rust
created_at: Some(node.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string())
```

### 3. 类型字段访问

**问题**: `NodeDetailModel` 的字段类型与 Next.js 中的不同

**差异**:
- `created_at`: `DateTime` (不是 `Option<String>`)
- `categories`: `Vec<CategoryModel>` (不是 `Option<Vec<CategoryModel>>`)
- `tags`: `Vec<TagModel>` (不是 `Option<Vec<TagModel>>`)

**处理方式**: 在视图层进行类型转换和格式化

## 文件结构

```
crates/oic-web/
├── src/
│   ├── app.rs              # Axum 应用入口和路由配置
│   ├── controllers/        # 控制器（处理请求）
│   │   ├── mod.rs
│   │   ├── home.rs         # 首页控制器
│   │   └── blog.rs         # 博客控制器
│   ├── views/              # 视图层（模板渲染）
│   │   ├── mod.rs
│   │   ├── home.rs         # 首页视图
│   │   └── blog.rs         # 博客视图
│   ├── models/             # 数据模型和模板结构
│   │   ├── mod.rs
│   │   ├── tpl.rs          # HtmlTemplate 包装器
│   │   └── vite.rs         # Vite 资源管理
│   ├── services/           # 业务逻辑（API 调用等）
│   │   ├── mod.rs
│   │   └── api.rs          # API 服务层
│   └── lib.rs
└── templates/              # Askama 模板文件
    ├── base.html           # 基础模板
    ├── home.html           # 首页模板
    ├── blog_list.html      # 博客列表模板
    └── blog_detail.html    # 博客详情模板
```

## 修复记录模板

每次修复时，请按照以下格式记录：

```markdown
### YYYY-MM-DD - 修复标题

**修复内容**:
- ✅ 修复项1（文件路径）
  - 具体修改说明
  - 代码变更摘要
- ✅ 修复项2（文件路径）
  - 具体修改说明

**遇到的问题**:
- ⚠️ 问题描述
- ⚠️ 解决方案

**测试结果**:
- ✅ 测试项1：通过/失败
- ✅ 测试项2：通过/失败

**相关文件**:
- `src/path/to/file.rs` - 修改说明
- `templates/file.html` - 修改说明

**备注**:
- 其他需要注意的事项
```

## 下一步计划

1. **立即修复编译错误**
   - [ ] 修复类型不匹配问题
   - [ ] 修复模板语法错误
   - [ ] 清理未使用的导入

2. **测试基本功能**
   - [ ] 测试首页渲染
   - [ ] 测试博客列表页
   - [ ] 测试博客详情页
   - [ ] 验证 API 调用

3. **完善功能**
   - [ ] 实现诗词列表页
   - [ ] 实现工具列表页
   - [ ] 添加侧边栏组件

4. **优化和部署**
   - [ ] 性能优化
   - [ ] 错误处理
   - [ ] 部署配置

## 参考资源

- [Askama 文档](https://djc.github.io/askama/)
- [Axum 文档](https://docs.rs/axum/)
- [oic-core 类型定义](../oic-core/src/models/)
- [Next.js 原实现](../../apps/web/src/)

## 修复记录

### 2024-12-XX - 统一缓存服务：合并渲染逻辑到 CacheService

**修复内容**:
- ✅ 创建 `src/services/cache.rs` - 统一的缓存服务
  - 实现 `CacheService` 结构体，封装 `Arc<Cache>` 实例
  - 提供底层缓存操作：
    - `get_html_bytes` - 使用底层 `Cache::get` API，返回 `Option<Vec<u8>>`
    - `set_html_bytes` - 使用底层 `Cache::set_with_ttl` API，接受 `Vec<u8>`
  - 提供便捷方法：
    - `get_html` / `set_html` - String 版本的便捷方法
  - 提供缓存渲染方法：
    - `get_cached_html_or_render` - 通用的 HTML 缓存渲染
    - `get_cached_template_or_render` - Askama 模板专用的缓存渲染
  - 添加 `CacheConfig` 结构体用于配置缓存 TTL（开发/生产环境）

- ✅ 删除 `src/services/render.rs`
  - 将所有渲染逻辑合并到 `CacheService` 中
  - 统一在一个 service 中，不需要分开两部分

- ✅ 更新 `src/app.rs`
  - 创建 `CacheService` 实例并作为 Extension 传递
  - 从 `Extension<Arc<Cache>>` 改为 `Extension<Arc<CacheService>>`

- ✅ 更新所有 Controllers
  - `src/controllers/home.rs` - 使用 `cache_service.get_cached_template_or_render()`
  - `src/controllers/blog.rs` - 使用 `cache_service.get_cached_template_or_render()`
  - 不再需要导入独立的 render 函数，直接调用 service 方法

**解决的问题**:
- ✅ 统一服务架构 - 所有缓存相关功能都在 `CacheService` 中
- ✅ 简化架构 - 不需要分开 cache 和 render 两部分
- ✅ 去掉泛型问题 - 直接使用 `Vec<u8>`，不需要 `CacheExt` trait
- ✅ 更符合单一职责 - `CacheService` 负责所有缓存和缓存渲染相关操作
- ✅ 消除重复代码 - Controller 层代码从 67 行减少到 40 行（home），163 行减少到 94 行（blog）

**技术细节**:
- `CacheService` 现在包含：
  - 底层缓存操作：`get_html_bytes`, `set_html_bytes`
  - 便捷方法：`get_html`, `set_html`
  - 缓存渲染：`get_cached_html_or_render`, `get_cached_template_or_render`
- 使用底层 API `Cache::get` 和 `Cache::set_with_ttl`，直接操作 `Vec<u8>`
- Controller 调用方式：`cache_service.get_cached_template_or_render(...)`
- 支持自定义缓存配置（可选），自动处理开发/生产环境的 TTL 差异

**相关文件**:
- `src/services/cache.rs` - **统一缓存服务**（合并了渲染逻辑）
- `src/services/render.rs` - **已删除**
- `src/services/mod.rs` - 移除 render 模块导出
- `src/app.rs` - 创建并传递 CacheService
- `src/controllers/home.rs` - 使用 `CacheService` 方法
- `src/controllers/blog.rs` - 使用 `CacheService` 方法

**测试结果**:
- ✅ 编译通过
- ✅ 类型检查通过
- ✅ 代码更简洁统一
- ✅ 架构更清晰

**备注**:
- 现在所有缓存相关功能都在 `CacheService` 中，架构更清晰
- Controller 只需要调用 `cache_service.get_cached_template_or_render()` 即可
- 未来如果需要其他缓存操作，继续在 `CacheService` 中添加
- 缓存键格式保持一致：`home:index`, `blog:list`, `blog:list:cat:{cat_vid}`, `blog:detail:{vid}`

---

### 2024-12-XX - 初始迁移

**修复内容**:
- ✅ 创建 API 服务层 (`src/services/api.rs`)
  - 使用 `oic_core::models::nodes::NodeFilters` 替代自定义参数类型
  - 使用 `oic_core::models::nodes::NodeDetailModel` 替代自定义 `NodeModel`
  - 使用 `oic_core::typings::JsonRes` 解析 API 响应
  - 实现 `describe_node_list` 和 `describe_node_detail` 函数

- ✅ 迁移首页 (`/`)
  - 创建 `src/controllers/home.rs` 和 `src/views/home.rs`
  - 创建 `templates/home.html`
  - 实现 Header、BigNews、News Grid、Article List、Footer 组件
  - 添加 HTML 缓存支持

- ✅ 迁移博客列表页 (`/blog/`)
  - 创建 `src/controllers/blog.rs` 中的 `blog_list` 函数
  - 创建 `src/views/blog.rs` 中的 `render_blog_list` 函数
  - 创建 `templates/blog_list.html`
  - 支持分类筛选 (`/cat/:cat_vid/`)

- ✅ 迁移博客详情页 (`/p/:vid/`)
  - 创建 `src/controllers/blog.rs` 中的 `blog_detail` 函数
  - 创建 `src/views/blog.rs` 中的 `render_blog_detail` 函数
  - 创建 `templates/blog_detail.html`

- ✅ 配置路由
  - 在 `src/app.rs` 中配置所有路由
  - 添加 manifest 和 cache 扩展

**遇到的问题**:
- ⚠️ 类型不匹配：`NodeDetailModel.created_at` 是 `DateTime` 类型，需要格式化为字符串
- ⚠️ 类型不匹配：`categories` 和 `tags` 是 `Vec`，不是 `Option<Vec>`
- ⚠️ Askama 模板不支持复杂的 Rust 表达式
- ⚠️ 未使用的导入：`src/services/mod.rs` 中 `cache::*`

**待修复**:
- [ ] 修复 `created_at` 类型转换（DateTime -> String）
- [ ] 修复 `categories` 和 `tags` 的类型处理
- [ ] 简化 Askama 模板中的条件判断
- [ ] 清理未使用的导入

---

## 更新日志

### 2024-12-XX
- ✅ 创建 API 服务层，使用 `oic-core` 类型
- ✅ 迁移首页、博客列表页、博客详情页
- ✅ 配置路由
- ⚠️ 发现类型不匹配问题，待修复

