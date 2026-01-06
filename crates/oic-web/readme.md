# lxage 博客

> 📝 **迁移进度**: 查看 [MIGRATION.md](./MIGRATION.md) 了解详细的迁移进度和待办事项

## 技术栈

主要技术： **axum** + **askama** + **oic_cache** + **alpine.js** + **tailwindcss** + **vitejs**

### 核心框架
- **axum**: Rust Web 框架，用于处理 HTTP 请求和路由
- **askama**: 类型安全的模板引擎（类似 Jinja2），用于服务端渲染 HTML
- **oic_cache**: 自定义缓存系统，支持 HTML 缓存和 SWR（Stale-While-Revalidate）策略
- **reqwest**: HTTP 客户端，用于调用 `api.lxage.com` 接口获取数据

### 前端技术
- **tailwindcss v4**: 主要界面样式全部使用 Tailwind CSS v4
- **alpine.js**: 少量交互优先使用 Alpine.js（轻量级，无需构建）
- **vitejs**: 前端资源构建工具，用于打包 CSS/JS 资源

### 数据获取
- 所有数据通过调用 `api.lxage.com` 接口获取
- 使用 `reqwest` 在服务端发起 HTTP 请求
- 响应数据通过 `askama` 模板渲染为 HTML

## 开发

```bash
# 开发模式（自动重载）
cargo watch -x run

# 生产构建
cargo build --release
```

## 从 Next.js 迁移说明

### 迁移可行性：✅ **完全可行**

原 Next.js SSR 应用（`apps/web`）可以完全迁移到 `crates/oic-web`，原因：

1. **SSR 页面迁移** ✅
   - Next.js 的 SSR 页面可以直接用 `askama` 模板替换
   - 所有服务端数据获取逻辑保持不变（调用相同 API）

2. **API 调用** ✅
   - Next.js 使用 `@repo/apis/server` 调用 `api.lxage.com`
   - Rust 使用 `reqwest` 调用相同的 API 端点
   - 数据格式和接口协议完全兼容

3. **样式系统** ✅
   - Tailwind CSS v4 可以直接使用
   - 无需修改 CSS 类名
   - 通过 Vite 构建 CSS 资源

4. **客户端交互** ⚠️
   - Next.js 的 React 组件需要转换为 Alpine.js
   - 状态管理（zustand）需要改为 Alpine.js 的 `x-data`
   - "加载更多"等功能需要用 Alpine.js 重写

### 迁移步骤建议

1. **页面路由迁移**
   - 首页：`/` → `controllers/home.rs`
   - 博客：`/blog/` → `controllers/blog.rs`
   - 诗词：`/poetry/` → `controllers/poetry.rs`
   - 工具：`/tool/` → `controllers/tool.rs`
   - 动态路由：`/cat/[catVid]` → `axum` 路径参数

2. **模板迁移**
   - React 组件 → Askama 模板
   - JSX → Jinja2 语法
   - Props → Template 结构体字段

3. **客户端交互迁移**
   - React hooks → Alpine.js `x-data`
   - useState → `x-data="{ count: 0 }"`
   - useEffect → Alpine.js 生命周期
   - 异步加载 → Alpine.js + fetch

4. **资源处理**
   - Next.js Image → 普通 `<img>` 或 CDN
   - MDX 内容 → Rust markdown 库（如 `pulldown-cmark`）
   - 静态资源 → Vite 构建后通过 `/assets` 路由提供

### 注意事项

- **性能优势**：Rust 服务端渲染性能远超 Node.js
- **内存占用**：Rust 二进制文件内存占用更小
- **部署简化**：单一二进制文件，无需 Node.js 运行时
- **缓存策略**：使用 `oic_cache` 实现页面级缓存，提升响应速度

## 项目结构

```
crates/oic-web/
├── src/
│   ├── app.rs           # Axum 应用入口和路由配置
│   ├── controllers/     # 控制器（处理请求）
│   ├── views/           # 视图层（模板渲染）
│   ├── models/          # 数据模型和模板结构
│   └── services/        # 业务逻辑（API 调用等）
└── templates/           # Askama 模板文件
```