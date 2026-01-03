use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

pub const ASSETS_PREFIX: &str = "/assets";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManifestChunk {
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

impl ManifestChunk {
  /// 解析指定目录的 manifest.json 文件
  pub fn from_path(file_path: String) -> HashMap<String, ManifestChunk> {
    let json_str = std::fs::read_to_string(file_path.as_str())
      .expect(&format!("Failed to Manifest file: {}", file_path.as_str()));

    let data: HashMap<String, ManifestChunk> = serde_json::from_str(&json_str)
      .expect(&format!("Failed to parse Manifest file: {}", file_path.as_str()));

    data
  }

  /// 根据 manifest 生成指定的JS css 代码块
  pub fn get_assets_by_name(
    manifests: HashMap<String, ManifestChunk>,
    name: &str
  ) -> ViteAssets {
    let m = manifests
      .into_values()
      .find(|item| item.name.eq(name))
      .unwrap_or(Self::default());

    let mut js = String::new();
    let mut css = String::new();

    let _ = writeln!(
      js,
      r#"<script type="module" src="{}/{}"></script>"#,
      ASSETS_PREFIX,
      m.file.as_str()
    );

    for css_file in m.css {
      let _ = writeln!(
        css,
        r#"<link rel="stylesheet" href="{}/{}">"#,
        ASSETS_PREFIX,
        css_file.as_str()
      );
    }
    
    ViteAssets { js, css }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ViteAssets {
  pub js: String,
  pub css: String,
}