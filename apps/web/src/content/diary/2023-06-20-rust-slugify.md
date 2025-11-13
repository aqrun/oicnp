---
title: Rust slugify
description: '将任何unicode字符串转为语义化的ascii码（方便博客一类应用中操作文件名或URL链接）'
slug: rust-slugify

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'slug']
---

将任何 unicode 字符串转为语义化的 ascii 码（方便博客一类应用中操作文件名或 URL 链接）。

生成的 slug 会保留 a-z、0-9 和中划线（-）。此外不会存在两个连续的中划线，也不会以中划线
开头或结尾。

## 主要用到的库：

- [slug 库](//docs.rs/slug)
- [deunicode](//docs.rs/deunicode)
- [regex](//docs.rs/regex)
- [once_cell](//docs.rs/once_cell)

## 示例：

```rust
assert_eq!(
    slugify_paths_without_date("My Test String!!!1!1"),
    "my-test-string-1-1"
);
assert_eq!(
    slugify_paths_without_date("2016-08-17-正文内容按换行用标签包装"),
    "zheng-wen-nei-rong-an-huan-xing-yong-biao-qian-bao-zhuang"
);
assert_eq!(
    slugify_paths_without_date("2015-07-17-移动页面基本结构"),
    "yi-dong-ye-mian-ji-ben-jie-gou"
);
```

## 功能实现

参考[zola](//www.getzola.org/)中的代码功能简化一下：

```rust
use regex::Regex;
use once_cell::sync::Lazy;
use std::path::Path;

// 正则分解文件名中的日期
// Based on https://regex101.com/r/H2n38Z/1/tests
// A regex parsing RFC3339 date followed by {_,-} and some characters
pub static RFC3339_DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<datetime>(\d{4})-(0[1-9]|1[0-2])
            -(0[1-9]|[12][0-9]|3[01])(T([01][0-9]|2[0-3]):([0-5][0-9])
            :([0-5][0-9]|60)(\.[0-9]+)?(Z|(\+|-)([01][0-9]|2[0-3])
            :([0-5][0-9])))?)\s?(_|-)(?P<slug>.+$)"
    ).unwrap()
});

/// 结构化文件名
/// 拆分为路径、文件名、后缀
pub struct CapturedFile {
    // 文件前缀路径
    pub parent: String,
    // 文件名
    pub file_stem: String,
    // 后缀
    pub ext: String,
}

impl CapturedFile {
    /// 将文件名结构转为字符串
    pub fn stringify(&self) -> String {
        if !self.file_stem.is_empty()
            && !self.ext.is_empty()
            && !self.parent.is_empty()
        {
            return format!("{}/{}.{}", self.parent, self.file_stem, self.ext);
        }

        if !self.file_stem.is_empty()
            && !self.ext.is_empty()
        {
            return format!("{}.{}", self.file_stem, self.ext);
        }

        String::from(&self.file_stem)
    }
}

/// 将文件路径转为结构化数据 CapturedFile
fn capture_file_name(s: &str) -> CapturedFile {
    let valid_str = s.replace("\\", "/");
    let p = Path::new(&valid_str);

    let mut parent = String::new();
    let mut file_stem = String::new();
    let mut ext = String::new();

    if let Some(parent_path) = p.parent() {
        parent = parent_path.to_str().unwrap_or("").to_string();
    }
    if let Some(stem) = p.file_stem() {
        file_stem = stem.to_str().unwrap_or("").to_string();
    }
    if let Some(extension) = p.extension() {
        ext = extension.to_str().unwrap_or("").to_string();
    }

    CapturedFile {
        parent,
        file_stem,
        ext,
    }
}

/// 语义化文件名并去除文件名的日期
pub fn slugify_paths_without_date(s: &str) -> String {
    let mut captured_file = capture_file_name(s);
    let mut file_path = String::from(captured_file.file_stem.as_str());

    // 正则匹配包含日期的文件名 无日期则不会匹配
    if let Some(caps) = RFC3339_DATE.captures(file_path.as_str()) {
        if let Some(s) = caps.name("slug") {
            file_path = s.as_str().to_string();
        }
    }
    // 将unicode转为ascii
    let res_slug = slug::slugify(file_path.as_str());
    // 更新文件名为 slug
    captured_file.file_stem = res_slug;
    captured_file.stringify()
}
```
