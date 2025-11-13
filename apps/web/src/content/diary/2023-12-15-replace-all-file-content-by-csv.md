---
title: 'Rust 根据修正后的国际化内容替换代码中所有的KEY'
description: '解析CSV文件，并使用rg查找所有匹配的文件再更新文件内容'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'rg', 'csv', '文件查找']
---

解析 CSV 文件，并使用 rg 查找所有匹配的文件再更新文件内容

```rust
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

static CSV: &'static str = "xxx.csv";
static PROJECT: &'static str = "/workspace/target-project-path";

#[tokio::main]
async fn main() {
  let i18n_data = get_all_i18n_data(PROJECT).await.unwrap();

  match handle_all_files(&i18n_data).await {
    Ok(_) => {
      println!("Complete");
    },
    Err(err) => {
      println!("Err {:?}", err);
    }
  };
}

async fn handle_all_files(data_items: &Vec<DataItem>) -> Result<String> {
  for item in data_items.iter() {
    for file in item.files.iter() {
      replace_file(file.as_str(), item.key.as_str(), &item.new_key.as_str()).await?;
    }
  }

  Ok(String::from(""))
}

async fn replace_file(file_path: &str, key: &str, new_key: &str) -> Result<String> {
  let mut file = File::open(file_path)?;
  let mut source_content = String::new();

  file.read_to_string(&mut source_content)?;

  let new_content = source_content.as_str().replace(key, new_key);

  std::fs::write(file_path, new_content.as_str())?;

  println!("更新文件：{}: {} -> {}", file_path, key, new_key);

  Ok(String::from(""))
}

// 获取Key源数据
async fn get_all_i18n_data(project: &str) -> Result<Vec<DataItem>>{
  let mut file = File::open(CSV)?;
  let mut csv = String::new();
  file.read_to_string(&mut csv)?;

  let mut data: Vec<DataItem> = Vec::new();
  let csv_lines = csv.split("\n").collect::<Vec<&str>>();

  for line in csv_lines.into_iter() {
    let line_items = line.split(",").collect::<Vec<&str>>();

    let key = line_items.get(2);
    let new_key = line_items.get(3);

    if key.is_some() && new_key.is_some() {
      let key = key.unwrap();
      let new_key = new_key.unwrap();

      if !key.eq(&"Key") {
        let files = find_files_by_project_path(project, *key).await;

        data.push(DataItem {
          key: String::from(*key),
          new_key: String::from(*new_key),
          files,
        });
      }
    }
  }

  Ok(data)
}


// 使用 max-depth=0 先查找根目录
// 再查找所有合法目录
pub async fn find_files_by_project_path(project_path: &str, i18n_key: &str) -> Vec<String> {
  // 查找根目录
  let mut files = find_files(i18n_key, project_path, true).await.unwrap();

  // 查找子目录
  if let Ok(entries) = std::fs::read_dir(project_path) {
      for entry in entries {
          if let Ok(entry) = entry {
              let path = entry.path();
              let path_str = path.to_str().unwrap();

              // 不包含配置文件排除的目录
              if path.is_dir() && !is_file_need_exclude(path_str).await {
                  let current_files = find_files(i18n_key, path_str, false).await.unwrap();
                  files = [files, current_files].concat();
              }
          }
      }
  }

  files
}

///
/// 查找所有内容匹配的文件
///
async fn find_files(i18n_key: &str, project_path: &str, is_root: bool) -> Result<Vec<String>> {
  let search = i18n_key;
  let mut search_path = PathBuf::from(project_path.trim_end_matches("/"));

  if !search_path.has_root() {
      search_path = PathBuf::from(project_path);

      if let Ok(item) = search_path.canonicalize() {
          search_path = item;
      }
  }

  let mut rg_cmd = format!(
      "rg -0 -s -e \"{}\" {}",
      search,
      search_path.to_str().unwrap()
  );

  // 根目录只查找一层
  if is_root {
      rg_cmd = format!(
          "rg -0 -s --max-depth=0 -e \"{}\" {}/*",
          search,
          search_path.to_str().unwrap()
      );
  }

  let output = if cfg!(target_os = "windows") {
      Command::new("cmd").args(["/C", &rg_cmd]).output()?
  } else {
      Command::new("sh").arg("-c").arg(&rg_cmd).output()?
  };

  let stdout = String::from_utf8(output.stdout)?;
  let arr: Vec<&str> = stdout.split("\n").collect();
  let valid_str_arr: Vec<&str> = arr
      .into_iter()
      .filter(|item| {
          let target = *item;

          if target.eq("") {
              return false;
          }
          return true;
      })
      .collect();

  let mut data: Vec<String> = Vec::new();

  for item in valid_str_arr.iter() {
      let item_arr: Vec<&str> = item.split("\0").collect();
      let file = *item_arr.get(0).unwrap();
      let file_item = String::from(file);

      // 排除指定文件
      if is_file_need_exclude(file_item.as_str()).await {
          continue;
      }

      // 重复的文件
      if !data.contains(&file_item) {
          data.push(file_item);
      }
  }

  Ok(data)
}


///
/// 检测是否是需要排除的文件
///
async fn is_file_need_exclude(filename: &str) -> bool {
  if filename.contains("binary file matches") {
      return true;
  }

  let exclude_files_names: Vec<&str> = vec![
    "node_modules",
    "config",
    ".node",
    "packages",
    "package.json",
    "package-lock.json",
  ];

  if exclude_files_names.is_empty() {
      return false;
  }

  let target = exclude_files_names.into_iter().find(|item| {
      return filename.ends_with(*item);
  });

  if let Some(_) = target {
      return true;
  }

  false
}

#[derive(Debug)]
pub struct DataItem {
  pub key: String,
  pub new_key: String,
  pub files: Vec<String>,
}
```
