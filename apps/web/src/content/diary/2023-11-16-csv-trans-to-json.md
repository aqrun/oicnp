---
title: 'Rust 读取CSV再转为所需的JSON格式'
description: 'Rust 读取CSV再转为所需的JSON格式'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'csv', 'json']
---

## 场景

CSV 源文件类似于：

![csv数据文件](https://cdn.oicnp.com/images/2023/20231116213202.png)

最后使用 2/3 列数据导出 JSON

![JSON](https://cdn.oicnp.com/images/2023/202311162133946.png)

## 代码实现

```rust
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const VALID_ROW_START: u32 = 2;
const CSV_SOURCE: &'static str = include_str!("/Users/aqrun/Documents/11.csv");
const DEFAULT_VALUE: &'static str = "";
const JSON_FILE: &'static str = "target/links.json";

fn main() {
  let links: Vec<ChannelLink> = handle_csv();
  println!("{:?}", &links);
  export_to_json(&links[..]);
}

pub fn export_to_json(links: &[ChannelLink]) {
  let mut results: BTreeMap<String, String> = BTreeMap::new();

  links.iter().for_each(|item| {
    results.insert(
      String::from(item.new_key.as_str()),
      String::from(item.value.as_str()),
    );
  });

  let content = serde_json::to_string_pretty(&results).unwrap();
  std::fs::write(JSON_FILE, &content).unwrap();
}

pub fn handle_csv() -> Vec<ChannelLink> {
  let mut links: Vec<ChannelLink> = Vec::new();

  let mut row_index = 0u32;
  CSV_SOURCE.split("\n")
    .for_each(|row| {
      if is_valid_row(row, row_index) {
        let items = row.split(",").collect::<Vec<&str>>();

        links.push(ChannelLink {
          app_name: get_column(&items[..], 0),
          key: get_column(&items[..], 1),
          new_key: get_column(&items[..], 2),
          value: get_column(&items[..], 3),
        });
      }

      row_index += 1;
    });

  links
}

fn get_column(items: &[&str], column: usize) -> String {
  let data = *items.get(column).unwrap_or(&DEFAULT_VALUE);
  String::from(data)
}

pub fn is_valid_row(row: &str, row_index: u32) -> bool {
  if row_index < VALID_ROW_START {
    return false;
  }
  if row.is_empty() {
    return false;
  }
  if row.starts_with("表格 1") {
    return false;
  }
  return true;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelLink {
  pub app_name: String,
  pub key: String,
  pub new_key: String,
  pub value: String,
}
```
