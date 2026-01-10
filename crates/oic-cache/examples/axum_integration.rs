use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use oic_cache::{Cache, CacheConfig};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    cache: Arc<Cache>,
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
        .route("/blog/post/{id}", get(show_post))
        .route("/api/cache/stats", get(cache_stats))
        .with_state(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");
    println!("Try:");
    println!("  - http://localhost:3000/");
    println!("  - http://localhost:3000/blog/post/123");
    println!("  - http://localhost:3000/api/cache/stats");

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Cache Demo Server</h1><p>Visit /blog/post/{id} to see cached content</p>")
}

async fn show_post(
    State(state): State<AppState>,
    Path(post_id): Path<u64>,
) -> Html<String> {
    let cache_key = format!("blog:post:{}", post_id);

    // 尝试从缓存获取
    match state.cache.get(&cache_key).await {
        Ok(Some(cached)) => {
            tracing::info!("Cache hit for post {}", post_id);
            return Html(String::from_utf8_lossy(&cached).to_string());
        }
        Ok(None) => {
            tracing::info!("Cache miss for post {}", post_id);
        }
        Err(e) => {
            tracing::warn!("Cache error for post {}: {}", post_id, e);
        }
    }

    // 缓存未命中，生成内容（模拟数据库查询）
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Post {}</title>
        </head>
        <body>
            <h1>Blog Post #{}</h1>
            <p>This is the content of post {}. It was generated at {}</p>
            <p><em>This content is cached for 1 hour.</em></p>
        </body>
        </html>
        "#,
        post_id,
        post_id,
        post_id,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    // 写入缓存（TTL 1 小时）
    if let Err(e) = state
        .cache
        .set_with_ttl(
            cache_key,
            bytes::Bytes::copy_from_slice(html.as_bytes()),
            "text/html".to_string(),
            3600,
        )
        .await
    {
        tracing::warn!("Failed to cache post {}: {}", post_id, e);
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

