/// cargo run --package oic_cache --example axum_ext
use axum::{
    extract::{Path, State},
    response::{Html, Json},
    routing::get,
    Router,
};
use oic_cache::{Cache, CacheConfig, CacheExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    cache: Arc<Cache>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BlogPost {
    id: u64,
    title: String,
    content: String,
    author: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建缓存实例
    let cache = Arc::new(Cache::new(CacheConfig::default()));
    let state = AppState { cache };

    // 构建路由
    let app = Router::new()
        .route("/", get(index))
        .route("/api/post/{id}", get(get_post_json))
        .route("/api/user/{id}", get(get_user_json))
        .route("/blog/post/{id}", get(show_post_html))
        .route("/api/cache/stats", get(cache_stats))
        .with_state(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");
    println!("Try:");
    println!("  - http://localhost:3000/");
    println!("  - http://localhost:3000/api/post/123");
    println!("  - http://localhost:3000/api/user/456");
    println!("  - http://localhost:3000/blog/post/123");
    println!("  - http://localhost:3000/api/cache/stats");

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(r#"
        <h1>Cache Extension Demo Server</h1>
        <p>This demo shows the convenience methods for caching JSON and HTML:</p>
        <ul>
            <li><a href="/api/post/123">GET /api/post/123</a> - JSON API with caching</li>
            <li><a href="/api/user/456">GET /api/user/456</a> - User JSON API</li>
            <li><a href="/blog/post/123">GET /blog/post/123</a> - HTML page with caching</li>
            <li><a href="/api/cache/stats">GET /api/cache/stats</a> - Cache statistics</li>
        </ul>
    "#)
}

/// 获取博客文章（JSON API，使用 set_json/get_json）
async fn get_post_json(
    State(state): State<AppState>,
    Path(post_id): Path<u64>,
) -> Json<BlogPost> {
    let cache_key = format!("api:post:{}", post_id);

    // 尝试从缓存获取（自动反序列化）
    if let Ok(Some(post)) = state.cache.get_json::<BlogPost>(&cache_key).await {
        tracing::info!("Cache hit for post {}", post_id);
        return Json(post);
    }

    tracing::info!("Cache miss for post {}", post_id);

    // 缓存未命中，生成内容（模拟数据库查询）
    let post = BlogPost {
        id: post_id,
        title: format!("Blog Post #{}", post_id),
        content: format!("This is the content of post {}. It was generated at {}.", 
            post_id,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ),
        author: "Alice".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // 写入缓存（自动序列化，TTL 1 小时）
    if let Err(e) = state.cache.set_json(cache_key, &post, 3600).await {
        tracing::warn!("Failed to cache post {}: {}", post_id, e);
    }

    Json(post)
}

/// 获取用户信息（JSON API）
async fn get_user_json(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
) -> Json<User> {
    let cache_key = format!("api:user:{}", user_id);

    // 尝试从缓存获取
    if let Ok(Some(user)) = state.cache.get_json::<User>(&cache_key).await {
        tracing::info!("Cache hit for user {}", user_id);
        return Json(user);
    }

    tracing::info!("Cache miss for user {}", user_id);

    // 模拟数据库查询
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };

    // 写入缓存（TTL 30 分钟）
    if let Err(e) = state.cache.set_json(cache_key, &user, 1800).await {
        tracing::warn!("Failed to cache user {}: {}", user_id, e);
    }

    Json(user)
}

/// 显示博客文章（HTML 页面，使用 set_html/get_html）
async fn show_post_html(
    State(state): State<AppState>,
    Path(post_id): Path<u64>,
) -> Html<String> {
    let cache_key = format!("blog:post:{}", post_id);

    // 尝试从缓存获取 HTML
    if let Ok(Some(html)) = state.cache.get_html(&cache_key).await {
        tracing::info!("Cache hit for post HTML {}", post_id);
        return Html(html);
    }

    tracing::info!("Cache miss for post HTML {}", post_id);

    // 缓存未命中，生成 HTML 内容
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Post {}</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 40px; }}
                h1 {{ color: #333; }}
                .meta {{ color: #666; font-size: 14px; }}
            </style>
        </head>
        <body>
            <h1>Blog Post #{}</h1>
            <div class="meta">
                <p>Generated at: {}</p>
                <p><em>This content is cached for 1 hour.</em></p>
            </div>
            <div class="content">
                <p>This is the content of post {}. It demonstrates the use of 
                <code>set_html()</code> and <code>get_html()</code> methods 
                for convenient HTML caching.</p>
            </div>
        </body>
        </html>
        "#,
        post_id,
        post_id,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        post_id
    );

    // 写入缓存（TTL 1 小时）
    if let Err(e) = state.cache.set_html(cache_key, &html, 3600).await {
        tracing::warn!("Failed to cache post HTML {}: {}", post_id, e);
    }

    Html(html)
}

async fn cache_stats(State(state): State<AppState>) -> Html<String> {
    let stats = state.cache.statistics().await;
    let hot_keys = state.cache.hot_keys(10).await;

    let hot_keys_html = if hot_keys.is_empty() {
        "<p>No hot keys yet.</p>".to_string()
    } else {
        hot_keys
            .iter()
            .map(|(key, count)| format!("<li>{}: {} accesses</li>", key, count))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Cache Statistics</title>
            <style>
                body {{ font-family: monospace; margin: 40px; }}
                h1 {{ color: #333; }}
                .stat {{ margin: 10px 0; padding: 10px; background: #f5f5f5; }}
                .hit-rate {{ font-size: 24px; font-weight: bold; color: #28a745; }}
            </style>
        </head>
        <body>
            <h1>Cache Statistics</h1>
            
            <div class="stat">
                <h2>Performance</h2>
                <p>Total Requests: {}</p>
                <p>Cache Hits: {}</p>
                <p>Cache Misses: {}</p>
                <p class="hit-rate">Hit Rate: {:.2}%</p>
            </div>
            
            <div class="stat">
                <h2>Capacity</h2>
                <p>Total Entries: {}</p>
                <p>Memory Usage: {:.2} MB</p>
                <p>Disk Usage: {:.2} GB</p>
            </div>
            
            <div class="stat">
                <h2>Hot Keys (Top 10)</h2>
                <ul>
                    {}
                </ul>
            </div>
        </body>
        </html>
        "#,
        stats.total_requests,
        stats.hits,
        stats.misses,
        stats.hit_rate * 100.0,
        stats.total_entries,
        stats.memory_usage_mb,
        stats.disk_usage_gb,
        hot_keys_html
    );

    Html(html)
}

