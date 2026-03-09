#![allow(dead_code)]
use crate::WebAppContext;
#[cfg(debug_assertions)]
use axum::routing::any;
use axum::{extract::Request, response::IntoResponse, Router};
use std::collections::{HashMap, HashSet};
use vite_rs_axum_0_8::ViteServe;

// Vite Manifest 结构（根据 Vite 官方文档）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct ViteManifestEntry {
    /// 此块/资源的输入文件名（若已知）
    pub src: String,
    /// 这个代码块/资源的输出文件名
    pub file: String,
    /// 该代码块导入的 CSS 文件列表
    /// 此字段仅在 JS 代码块中存在。
    pub css: Vec<String>,
    /// 该代码块导入的资源文件列表，不包括 CSS 文件
    /// 此字段仅在 JS 代码块中存在。
    pub assets: Vec<String>,
    /// 该代码块或资源是否为入口点
    #[serde(rename(deserialize = "isEntry", serialize = "isEntry"))]
    pub is_entry: bool,
    /// 此块/资源的名称（如已知）
    pub name: String,
    /// 该代码块是否为动态入口点
    /// 此字段仅在 JS 代码块中存在。
    #[serde(rename(deserialize = "isDynamicEntry", serialize = "isDynamicEntry"))]
    pub is_dynamic_entry: bool,
    /// 该代码块静态导入的代码块列表
    /// 这些值是 manifest 中的键。此字段仅在 JS 代码块中存在。
    pub imports: Vec<String>,
    /// 该代码块动态导入的代码块列表
    /// 这些值是 manifest 中的键。此字段仅在 JS 代码块中存在。
    #[serde(rename(deserialize = "dynamicImports", serialize = "dynamicImports"))]
    pub dynamic_imports: Vec<String>,
}

type ViteManifest = HashMap<String, ViteManifestEntry>;

/// 资源文件结构体，用于在模板中引用 Vite 编译后的 JS 和 CSS 文件
///
/// 支持多文件入口（多个 JS 和 CSS 文件），自动处理依赖关系。
///
/// # 字段说明
///
/// - `js_files`: JavaScript 文件路径列表（包括入口文件和静态导入的依赖文件）
/// - `css_files`: CSS 文件路径列表（在 Debug 模式下通常为空，CSS 通过 JS import 加载）
///
/// # 示例
///
/// ```rust
/// use vite::AssetFiles;
///
/// // 使用默认配置（推荐）
/// let assets = AssetFiles::default();
///
/// // 在 Askama 模板中使用
/// // {% for js_file in assets.js_files %}
/// // <script type="module" src="{{ js_file }}"></script>
/// // {% endfor %}
/// ```
#[derive(Clone)]
pub struct AssetFiles {
    /// JavaScript 文件路径列表（包括入口文件和静态导入的文件）
    pub js_files: Vec<String>,
    /// CSS 文件路径列表
    pub css_files: Vec<String>,
}

impl Default for AssetFiles {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            // Debug 模式：按 Vite 官方约定
            // Vite dev server 会自动处理模块解析
            // CSS 通过 JS import 加载，不需要单独 <link>
            Self {
                js_files: vec!["/src/main.js".to_string()],
                css_files: vec![], // CSS 通过 JS import 加载，不需要单独 <link>
            }
        }
        #[cfg(not(debug_assertions))]
        {
            // Release 模式：尝试从文件系统读取 manifest
            // 如果没有找到，会使用默认值
            let (js_files, css_files) = Self::get_paths_from_manifest(|| None);
            Self {
                js_files,
                css_files,
            }
        }
    }
}

impl AssetFiles {
    /// 获取资源文件路径（JS 和 CSS）
    /// - Debug 模式：使用固定的源文件路径（main.js），CSS 通过 import 加载
    /// - Release 模式：从 manifest 获取编译后的路径
    ///
    /// # Arguments
    /// * `get_manifest` - 可选函数，用于在 release 模式下获取 manifest 内容
    pub fn new<F>(get_manifest: Option<F>) -> Self
    where
        F: FnOnce() -> Option<String>,
    {
        #[cfg(debug_assertions)]
        {
            // Debug 模式：忽略 manifest 加载器，使用固定路径
            Self::default()
        }
        #[cfg(not(debug_assertions))]
        {
            // Release 模式：使用提供的 manifest 加载器，如果没有则尝试从文件系统读取
            let (js_files, css_files) = if let Some(get_manifest_fn) = get_manifest {
                Self::get_paths_from_manifest(get_manifest_fn)
            } else {
                Self::get_paths_from_manifest(|| None)
            };
            Self {
                js_files,
                css_files,
            }
        }
    }

    /// 使用自定义 manifest 加载器创建 AssetFiles
    ///
    /// # Arguments
    /// * `get_manifest` - 函数，用于获取 manifest 内容
    pub fn with_manifest_loader<F>(get_manifest: F) -> Self
    where
        F: FnOnce() -> Option<String>,
    {
        Self::new(Some(get_manifest))
    }

    /// 获取第一个 JS 文件路径（向后兼容）
    pub fn js_path(&self) -> &str {
        self.js_files.first().map(|s| s.as_str()).unwrap_or("")
    }

    /// 获取第一个 CSS 文件路径（向后兼容）
    pub fn css_path(&self) -> &str {
        self.css_files.first().map(|s| s.as_str()).unwrap_or("")
    }

    /// 检查 CSS 文件列表是否为空（用于模板）
    pub fn has_css_files(&self) -> bool {
        !self.css_files.is_empty()
    }

    #[cfg(not(debug_assertions))]
    fn get_paths_from_manifest<F>(get_manifest: F) -> (Vec<String>, Vec<String>)
    where
        F: FnOnce() -> Option<String>,
    {
        // 尝试从传入的函数获取 manifest
        let manifest_str_opt = get_manifest().or_else(|| {
            // 如果函数返回 None，尝试从文件系统读取
            // 构建输出目录通常是 ./app/dist
            let manifest_paths = vec![
                "./apps/web-app/dist/.vite/manifest.json",
                "./app/dist/.vite/manifest.json",
                "./dist/.vite/manifest.json",
                "../app/dist/.vite/manifest.json",
            ];

            for path in manifest_paths {
                if let Ok(content) = std::fs::read_to_string(path) {
                    return Some(content);
                }
            }
            None
        });

        if let Some(manifest_str) = manifest_str_opt {
            if let Ok(manifest) = serde_json::from_str::<ViteManifest>(&manifest_str) {
                if let Some((js_files, css_files)) = Self::extract_paths_from_manifest(&manifest) {
                    return (js_files, css_files);
                }
            }
        }

        // 如果无法从 manifest 获取，使用默认值
        // 这应该不会发生，因为 vite-rs 会生成 manifest
        eprintln!("Warning: Could not read manifest.json, using default paths");
        (
            vec!["/assets/main.js".to_string()],
            vec!["/style.css".to_string()],
        )
    }

    /// 从 manifest 中提取路径（提取逻辑的共用部分，供 debug 和 release 模式使用）
    fn extract_paths_from_manifest(manifest: &ViteManifest) -> Option<(Vec<String>, Vec<String>)> {
        // 查找入口文件
        // 优先级：
        // 1. 查找标记为 isEntry: true 的条目
        // 2. 查找常见的入口文件名（app.js, main.tsx, index.js 等）
        // 3. 使用第一个条目
        let mut entry_key = None;
        let mut entry_value = None;

        // 首先查找标记为入口的条目
        for (key, entry) in manifest.iter() {
            if entry.is_entry {
                entry_key = Some(key);
                entry_value = Some(entry);
                break;
            }
        }

        // 如果没有找到 isEntry，查找常见的入口文件名
        if entry_key.is_none() {
            let common_entry_names = [
                "app.js",
                "main.tsx",
                "main.ts",
                "main.js",
                "index.js",
                "index.tsx",
                "index.ts",
            ];
            for (key, entry) in manifest.iter() {
                if common_entry_names
                    .iter()
                    .any(|&name| key.ends_with(name) || key == name)
                {
                    entry_key = Some(key);
                    entry_value = Some(entry);
                    break;
                }
            }
        }

        // 如果还没找到，使用第一个条目
        if entry_key.is_none() {
            if let Some((key, entry)) = manifest.iter().next() {
                entry_key = Some(key);
                entry_value = Some(entry);
            }
        }

        // 获取所有 JS 和 CSS 路径（包括入口文件和依赖）
        if let (Some(entry_key), Some(entry)) = (entry_key, entry_value) {
            let mut js_files = Vec::new();
            let mut css_files = Vec::new();

            // 收集所有相关的 JS 文件（入口文件 + 静态导入的文件）
            let mut collected_keys = HashSet::new();
            Self::collect_js_files(manifest, entry_key, &mut js_files, &mut collected_keys);

            // 收集所有 CSS 文件
            // 1. 从入口文件的 css 字段获取
            for css in &entry.css {
                css_files.push(format!("/public/{}", css));
            }

            // 2. 从静态导入的文件中收集 CSS
            for import_key in &entry.imports {
                if let Some(import_entry) = manifest.get(import_key) {
                    for css in &import_entry.css {
                        let css_path = format!("/public/{}", css);
                        if !css_files.contains(&css_path) {
                            css_files.push(css_path);
                        }
                    }
                }
            }

            // 3. 如果没有找到 CSS，查找独立的 CSS 条目
            if css_files.is_empty() {
                let entry_name = entry_key;
                let base_name = entry_name
                    .rsplit('/')
                    .next()
                    .unwrap_or(entry_name)
                    .split('.')
                    .next()
                    .unwrap_or("");

                // 先查找与入口文件同名的 CSS
                let css_name = format!("{}.css", base_name);
                let css_entry = manifest
                    .iter()
                    .find(|(key, _)| key.ends_with(&css_name) || **key == css_name);

                if let Some((_, css_entry)) = css_entry {
                    css_files.push(format!("/public/{}", css_entry.file));
                } else {
                    // 如果没找到，使用第一个 CSS 条目
                    if let Some((_, css_entry)) =
                        manifest.iter().find(|(key, _)| key.ends_with(".css"))
                    {
                        css_files.push(format!("/public/{}", css_entry.file));
                    }
                }
            }

            // 如果 JS 路径为空，至少添加入口文件
            if js_files.is_empty() {
                js_files.push(format!("/public/{}", entry.file));
            }

            return Some((js_files, css_files));
        }
        None
    }

    /// 递归收集所有相关的 JS 文件（入口文件 + 静态导入的文件）
    fn collect_js_files(
        manifest: &ViteManifest,
        key: &str,
        js_files: &mut Vec<String>,
        collected_keys: &mut HashSet<String>,
    ) {
        // 避免重复收集
        if collected_keys.contains(key) {
            return;
        }
        collected_keys.insert(key.to_string());

        if let Some(entry) = manifest.get(key) {
            // 添加当前文件的 JS 路径
            js_files.push(format!("/public/{}", entry.file));

            // 递归收集静态导入的文件
            for import_key in &entry.imports {
                Self::collect_js_files(manifest, import_key, js_files, collected_keys);
            }
        }
    }
}

/// 检查路径是否是静态资源
fn is_static_asset(path: &str) -> bool {
    // 静态资源路径模式
    path.starts_with("/assets/")
        || path.starts_with("/@vite/")
        || path.starts_with("/node_modules/")
        || path == "/main.js"
        || path == "/main.ts"
        || path == "/style.css"
        || path == "/@react-refresh"
        || path.ends_with(".js")
        || path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with("env.mjs")
        || path.ends_with(".css")
        || path.ends_with(".less")
        || path.ends_with(".png")
        || path.ends_with(".jpg")
        || path.ends_with(".jpeg")
        || path.ends_with(".gif")
        || path.ends_with(".svg")
        || path.ends_with(".ico")
        || path.ends_with(".woff")
        || path.ends_with(".woff2")
        || path.ends_with(".ttf")
        || path.ends_with(".eot")
        || path.contains("@vitejs")
}

/// 智能静态资源处理器
/// 只处理静态资源请求，其他请求返回 404，避免拦截 Askama 路由
async fn handle_static_assets(req: Request, vite_serve: ViteServe) -> impl IntoResponse {
    let path = req.uri().path();

    // 检查是否是静态资源路径
    if is_static_asset(path) {
        vite_serve.serve(req).await
    } else {
        // 不是静态资源，返回 404
        (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response()
    }
}

/// 配置 Axum 静态资源路由
///
/// 此函数会根据编译模式自动配置合适的路由：
///
/// - **Debug 模式**: 使用智能处理器，只处理静态资源请求，避免拦截其他路由
///   - 匹配所有静态资源路径（`/assets/*`, `/@vite/*`, `/main.js`, `/style.css` 等）
///   - 支持 Vite HMR（Hot Module Replacement）
///   - 不会拦截其他 Askama 路由（如 `/about`, `/contact` 等）
///
/// - **Release 模式**: 只匹配编译后的静态资源路径
///   - `/assets/*` - 编译后的 JS 文件
///   - `/style.css` - CSS 文件（可能在根目录）
///
/// # 参数
///
/// * `vite_serve` - ViteServe 实例，用于处理静态资源请求
///
/// # 返回值
///
/// 返回配置好的 Axum Router，应该使用 `.merge()` 合并到主应用中
///
/// # 示例
///
/// ```rust
/// use vite::static_assets_router;
/// use vite_rs_axum_0_8::ViteServe;
///
/// let vite_serve = ViteServe::new(Assets::boxed());
/// let app = Router::new()
///     .route("/", get(handle_index))
///     .merge(static_assets_router(vite_serve));  // 必须放在其他路由之后
/// ```
///
/// # 注意事项
///
/// - 此路由应该放在所有其他路由**之后**，作为最后的 fallback
/// - Debug 模式下会使用 `/{*path}` 通配路由，但会智能判断是否为静态资源
pub fn static_assets_router(vite_serve: ViteServe) -> Router<WebAppContext> {
    #[cfg(debug_assertions)]
    {
        // Debug 模式：使用智能处理器，只处理静态资源请求
        // 这样不会拦截其他 Askama 路由（如 /about, /contact 等）
        // 注意：这个路由必须放在所有其他路由之后，作为最后的 fallback
        Router::new().route(
            "/{*path}",
            any(move |req: Request| {
                let vite_serve = vite_serve.clone();
                async move { handle_static_assets(req, vite_serve).await }
            }),
        )
    }

    #[cfg(not(debug_assertions))]
    {
        // Release 模式：处理嵌入的静态资源
        // 注意：/public/* 路径由文件系统 ServeDir 处理，不在这里
        // 只处理编译后的资源：/assets/* (JS/CSS), /.vite/* (manifest)
        Router::new()
            .route_service("/assets/{*path}", vite_serve.clone())
            .route_service("/.vite", vite_serve.clone())
    }
}
