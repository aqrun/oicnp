use serde::{Deserialize, Serialize};
use regex::Regex;
use once_cell::sync::Lazy;
use std::path::{Path};

// Based on https://regex101.com/r/H2n38Z/1/tests
// A regex parsing RFC3339 date followed by {_,-} and some characters
pub static RFC3339_DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<datetime>(\d{4})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])(T([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(Z|(\+|-)([01][0-9]|2[0-3]):([0-5][0-9])))?)\s?(_|-)(?P<slug>.+$)"
    ).unwrap()
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SlugifyStrategy {
    /// Classic slugification, the default
    On,
    /// No slugification, only remove unsafe characters for filepaths/urls
    Safe,
    /// Nothing is changed, hope for the best!
    Off,
}

impl Default for SlugifyStrategy {
    fn default() -> Self {
        SlugifyStrategy::On
    }
}

fn strip_chars(s: &str, chars: &str) -> String {
    let mut sanitized_string = s.to_string();
    sanitized_string.retain(|c| !chars.contains(c));
    sanitized_string
}

fn strip_invalid_paths_chars(s: &str) -> String {
    // NTFS forbidden characters : https://gist.github.com/doctaphred/d01d05291546186941e1b7ddc02034d3
    // Also we need to trim whitespaces and `.` from the end of filename
    let trimmed = s.trim_end_matches(|c| c == ' ' || c == '.');
    strip_chars(trimmed, r#"<>:"/\|?*"#)
}

pub fn slugify_paths(s: &str, strategy: SlugifyStrategy) -> String {
    match strategy {
        SlugifyStrategy::On => slug::slugify(s),
        SlugifyStrategy::Safe => strip_invalid_paths_chars(s),
        SlugifyStrategy::Off => s.to_string(),
    }
}

pub struct CapturedFile {
    // 文件前缀路径
    pub parent: String,
    // 文件名
    pub file_stem: String,
    // 后缀
    pub ext: String,
}

impl CapturedFile {
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
pub fn capture_file_name(s: &str) -> CapturedFile {
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

/// 去除文件名的日期
pub fn slugify_paths_without_date(s: &str) -> (String, String) {
    let mut date_time = String::new();
    let captured_file = capture_file_name(s);
    let mut file_path = String::from(captured_file.file_stem.as_str());

    // 正则匹配包含日期的文件名 无日期则不会匹配
    if let Some(caps) = RFC3339_DATE.captures(file_path.as_str()) {
        if let Some(s) = caps.name("datetime") {
            date_time = s.as_str().to_string();
        }
        if let Some(s) = caps.name("slug") {
            file_path = s.as_str().to_string();
        }
    }
    // 将unicode转为ascii
    let res_slug = slug::slugify( file_path.as_str());
    (date_time, res_slug)
}

pub fn slugify_anchors(s: &str, strategy: SlugifyStrategy) -> String {
    match strategy {
        SlugifyStrategy::On => slug::slugify(s),
        SlugifyStrategy::Safe | SlugifyStrategy::Off => {
            s.replace(|c: char| c.is_ascii_whitespace(), "_")
        }
    }
}
