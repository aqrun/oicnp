# 部署说明

## API 转发配置

本项目采用**开发/生产环境分离**的 API 转发策略：

- **开发环境**：使用 Next.js `rewrites` 自动转发
- **生产环境**：使用 Nginx 反向代理（推荐）

### 开发环境

开发环境会自动使用 Next.js 的 `rewrites` 功能，无需额外配置。

启动开发服务器：
```bash
cd apps/web
npm run dev
```

前端请求 `/api/v1/xxx` 会自动转发到后端 API（默认 `http://localhost:5150`）。

### 生产环境

生产环境建议使用 Nginx 反向代理，具有以下优势：

1. **性能更好**：Nginx 是专业的反向代理服务器，性能优于 Node.js 层面的转发
2. **资源占用更少**：不占用 Next.js 服务器资源
3. **更灵活**：可以配置缓存、限流、SSL 等高级功能
4. **更稳定**：Nginx 经过大量生产环境验证

#### Nginx 配置步骤

1. **复制配置文件**
   ```bash
   cp nginx.conf.example /etc/nginx/sites-available/oicnp
   # 或
   cp nginx.conf.example /etc/nginx/conf.d/oicnp.conf
   ```

2. **修改配置**
   - 将 `your-domain.com` 替换为你的实际域名
   - 确认端口号：
     - Next.js 应用端口（默认 3000）
     - 后端 API 端口（默认 5150）

3. **启用配置**
   ```bash
   # 如果使用 sites-available/sites-enabled
   ln -s /etc/nginx/sites-available/oicnp /etc/nginx/sites-enabled/
   
   # 测试配置
   nginx -t
   
   # 重载配置
   nginx -s reload
   # 或
   systemctl reload nginx
   ```

4. **验证**
   - 访问 `http://your-domain.com` 应该能看到前端页面
   - 访问 `http://your-domain.com/api/v1/xxx` 应该能访问到后端 API

#### 环境变量配置

确保设置了正确的环境变量：

```bash
# 前端应用（Next.js）
NEXT_PUBLIC_OICNP_API_URI=http://localhost:5150  # 开发环境
# 或
NEXT_PUBLIC_OICNP_API_URI=https://api.your-domain.com  # 生产环境（如果 API 有独立域名）

# 后端应用
API_PORT=5150
DATABASE_URL=postgres://...
```

**注意**：如果生产环境使用 Nginx 转发，前端代码中的 API 请求应该使用相对路径 `/api/...`，这样会通过 Nginx 转发到后端。

### 其他部署方式

#### 使用 Docker

如果使用 Docker 部署，可以在 `docker-compose.yml` 中配置 Nginx 服务：

```yaml
services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/conf.d/default.conf
    depends_on:
      - web
      - api
```

#### 使用云平台

- **Vercel/Netlify**：这些平台支持 Next.js rewrites，可以直接使用
- **自建服务器**：推荐使用 Nginx 反向代理

### 故障排查

1. **API 请求 404**
   - 检查 Nginx 配置中的 `proxy_pass` 地址是否正确
   - 检查后端服务是否正常运行
   - 查看 Nginx 错误日志：`tail -f /var/log/nginx/error.log`

2. **CORS 错误**
   - 检查后端 CORS 配置
   - 或在 Nginx 配置中添加 CORS 头（见 `nginx.conf.example`）

3. **开发环境转发不工作**
   - 确认 `NODE_ENV=development`
   - 检查 `next.config.ts` 中的 rewrites 配置

