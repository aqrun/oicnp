use std::collections::{HashMap, HashSet};

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
        let (js_files, css_files) = Self::get_paths_from_manifest(|| None);
        Self { js_files, css_files }
    }
}

impl AssetFiles {
    fn get_paths_from_manifest<F>(get_manifest: F) -> (Vec<String>, Vec<String>)
    where
        F: FnOnce() -> Option<String>,
    {
        // 尝试从传入的函数获取 manifest
        let manifest_str_opt = get_manifest().or_else(|| {
            // 如果函数返回 None，尝试从文件系统读取
            // 构建输出目录通常是 ./app/dist
            let manifest_paths = vec![
                "./apps/backend-app/build/.vite/manifest.json",
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
            let common_entry_names = ["app.js", "main.tsx", "main.ts", "main.js", "index.js", "index.tsx", "index.ts"];
            for (key, entry) in manifest.iter() {
                if common_entry_names.iter().any(|&name| key.ends_with(name) || key == name) {
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
                let css_entry = manifest.iter().find(|(key, _)| {
                    key.ends_with(&css_name) || **key == css_name
                });
                
                if let Some((_, css_entry)) = css_entry {
                    css_files.push(format!("/public/{}", css_entry.file));
                } else {
                    // 如果没找到，使用第一个 CSS 条目
                    if let Some((_, css_entry)) = manifest.iter().find(|(key, _)| key.ends_with(".css")) {
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
