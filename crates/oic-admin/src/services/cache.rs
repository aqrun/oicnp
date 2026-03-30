/// 简化缓存调用的宏
/// 
/// 该宏简化 `get_cached_or_render` 的调用，减少代码冗余。
/// 宏直接返回 `Response`，包含错误处理逻辑。
/// 
/// **开发模式行为**：在开发模式下不使用缓存，直接渲染以便快速发现错误。
/// **生产模式行为**：使用缓存提高性能。
/// 
/// # 使用示例
/// 
/// ```rust
/// // 基础用法
/// async fn index(...) -> impl IntoResponse {
///     cached!(&*cache, "home:index", render_home_index(manifest.clone()))
/// }
/// 
/// // 自定义 TTL
/// async fn blog_list(...) -> impl IntoResponse {
///     cached!(&*cache, "blog:list", render_blog_list(None, manifest.clone()), 7200)
/// }
/// ```
#[macro_export]
macro_rules! cached {
    ($cache:expr, $key:expr, $render:expr) => {{
        // 开发模式：不使用缓存，直接渲染以便快速发现错误
        #[cfg(debug_assertions)]
        {
            let _ = &$cache;
            let _ = oic_core::services::cache_client::get_cached_or_render(
                $cache.as_ref(),
                "dev:none",
                move || async move { Ok(bytes::Bytes::from("")) },
                None,
            ).await;
            
            match $render.await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => {
                    // 使用 tracing 记录错误
                    tracing::error!(key = $key, error = %e, "Render failed in development mode");
                    
                    // 输出详细的错误信息到控制台
                    eprintln!("\n❌ RENDER ERROR [{}]", $key);
                    eprintln!("   Error: {}", e);
                    // 显示错误链
                    for cause in e.chain().skip(1) {
                        eprintln!("   Caused by: {}", cause);
                    }
                    // 显示完整的调试信息（包括 backtrace，如果可用）
                    eprintln!("   Debug info:\n{:?}", e);
                    eprintln!();
                    
                    crate::views::handle_error(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        e
                    ).await
                }
            }
        }
        
        // 生产模式：使用缓存
        #[cfg(not(debug_assertions))]
        {
            let render_future = $render;
            match oic_core::services::cache_client::get_cached_or_render(
                $cache.as_ref(),
                $key,
                move || render_future,
                None,
            ).await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => crate::views::handle_error(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    e
                ).await,
            }
        }
    }};
    
    // 支持自定义 TTL
    ($cache:expr, $key:expr, $render:expr, $ttl:expr) => {{
        // 开发模式：不使用缓存，直接渲染以便快速发现错误
        #[cfg(debug_assertions)]
        {
            let _ = &$cache;
            let _ = oic_core::services::cache_client::get_cached_or_render(
                $cache.as_ref(),
                "dev:none",
                move || async move { Ok(bytes::Bytes::from("")) },
                None,
            ).await;

            match $render.await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => {
                    // 使用 tracing 记录错误
                    tracing::error!(key = $key, ttl = $ttl, error = %e, "Render failed in development mode");
                    
                    // 输出详细的错误信息到控制台
                    eprintln!("\n❌ RENDER ERROR [{}] (TTL: {})", $key, $ttl);
                    eprintln!("   Error: {}", e);
                    // 显示错误链
                    for cause in e.chain().skip(1) {
                        eprintln!("   Caused by: {}", cause);
                    }
                    // 显示完整的调试信息（包括 backtrace，如果可用）
                    eprintln!("   Debug info:\n{:?}", e);
                    eprintln!();
                    
                    crate::views::handle_error(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        e
                    ).await
                }
            }
        }
        
        // 生产模式：使用缓存
        #[cfg(not(debug_assertions))]
        {
            let render_future = $render;
            let config = oic_core::services::cache_client::CacheConfig {
                dev_ttl: $ttl,
                prod_ttl: $ttl,
            };
            match oic_core::services::cache_client::get_cached_or_render(
                $cache.as_ref(),
                $key,
                move || render_future,
                Some(config),
            ).await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => crate::views::handle_error(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    e
                ).await,
            }
        }
    }};
}

