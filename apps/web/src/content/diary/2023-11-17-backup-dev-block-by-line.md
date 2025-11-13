---
title: 'Rust 按行备份文件代码块并还原'
description: 'Rust 按行备份文件代码块并还原'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'dev', 'pear']
---

## 场景

如下示例代码，将 `PEAR_BEGIN` 和 `PEAR_END` 解析出来并缓存，下次再插入文件原位置

```js
function add() {
  let a = 1;
  let b = 2;
  let c = 3;
  let d = 5;
  let e = 0;

  // PEAR_BEGIN:测试代码1
  // 这是测试代码
  // 可以随便放东西
  let q = 100;
  let p = 200;
  c = q + p;
  console.log('result---');
  // PEAR_END
  for (let i = 0; i < 100; i++) {
    e += i;
    // PEAR_BEGIN:测试代码2
    let name = 'alex';
    // PEAR_END
  }

  return e;
}
```

解析后的数据：

```json
[
  {
    "id": "16",
    "name": "测试代码1",
    "line": 9,
    "file_path": "target/code1.js",
    "total_dev_line": 8,
    "content": "  // PEAR_BEGIN:测试代码1\n  // 这是测试代码\n  // 可以随便放东西\n  let q = 100;\n  let p = 200;\n  c = q + p;\n  console.log(\"result---\");\n  // PEAR_END"
  },
  {
    "id": "21",
    "name": "测试代码2",
    "line": 19,
    "file_path": "target/code1.js",
    "total_dev_line": 3,
    "content": "    // PEAR_BEGIN:测试代码2\n    let name = \"alex\";\n    // PEAR_END"
  }
]
```

## 代码实现

```rust
//! 按行插入指定内容

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

const DEV_BEGIN_SIGN: &'static str = "// PEAR_BEGIN:";
const SOURCE: &'static str = "target/code1.js";
const DB_FILE: &'static str = "target/code.db";

fn main() {
  // stop_dev(SOURCE).unwrap();
  start_dev().unwrap();
}

/// 开始开发模式 插入开发代码块到源文件中
pub fn start_dev() -> Result<()> {
  let db_content = std::fs::read_to_string(DB_FILE)?;
  let db_data: Vec<PearBlock> = serde_json::from_str(db_content.as_str())?;

  let all_files = get_all_file_from_db(&db_data);
  let mut errs: Vec<String> = Vec::new();

  all_files.iter().for_each(|file_path| {
    let dev_blocks = db_data.iter().filter(|item| {
      item.file_path.as_str().eq(file_path.as_str())
    }).collect::<Vec<&PearBlock>>();

    if let Err(err) = insert_dev_data_to_file(file_path.as_str(), &dev_blocks) {
      errs.push(format!("文件还原失败: {:?}", err));
    }
  });

  if errs.len() > 0 {
    return Err(anyhow!("{}", errs.join("\n")));
  }

  Ok(())
}

/// 结束开发模式 保存开发代码块到数据库
pub fn stop_dev(source_file_path: &str) -> Result<()> {
  let source_file_data = parse_source_file_data(source_file_path)?;
  revert_source_file(&source_file_data)?;
  write_to_db(&source_file_data)?;
  Ok(())
}

pub fn insert_dev_data_to_file(file_path: &str, dev_blocks: &Vec<&PearBlock>) -> Result<()> {
  let mut source_content: String = String::from("");

  if let Ok(item) = std::fs::read_to_string(file_path) {
    source_content = item;
  }

  let mut source_content_arr: Vec<String> = Vec::new();

  let mut line_index = 0u32;
  source_content.split("\n").for_each(|line| {
    if let Some(block) = dev_blocks.iter().find(|item| {
      item.line == line_index
    }) {
      block.content.split("\n").for_each(|i| {
        source_content_arr.push(String::from(i));
      });
      line_index += block.total_dev_line;
    }

    source_content_arr.push(String::from(line));

    line_index += 1;
  });

  let contents = source_content_arr.join("\n");
  std::fs::write(file_path, contents.as_str())?;
  Ok(())
}

pub fn get_all_file_from_db(db_data: &[PearBlock]) -> Vec<String> {
  let mut all_files: Vec<String> = Vec::new();

  db_data.iter().for_each(|item| {
    let target = all_files.iter().find(|n| {
      n.as_str().eq(item.file_path.as_str())
    });

    if target.is_none() {
      all_files.push(String::from(item.file_path.as_str()));
    }
  });

  all_files
}

pub fn write_to_db(source_file_data: &SourceFileData) -> Result<String> {
  let contents = serde_json::to_string_pretty(&source_file_data.dev_blocks)?;
  std::fs::write(DB_FILE, contents.as_str())?;
  Ok(String::from("写入数据库成功"))
}

pub fn revert_source_file(source_file_data: &SourceFileData) -> Result<String> {
  let file_path = source_file_data.file_path.as_str();
  std::fs::write(file_path, source_file_data.contents.as_str())?;

  Ok(String::from("源文件已还原"))
}

/// 查找指定文件所有的开发代码块
pub fn parse_source_file_data(source_file_path: &str) -> Result<SourceFileData> {
  let source_content = std::fs::read_to_string(source_file_path)?;

  // 当前文件所有代码块数据
  let mut blocks: Vec<PearBlock> = Vec::new();
  // 源文件中有效内容
  let mut normal_code_source: Vec<String> = Vec::new();
  // 临时存储一个代码块内容
  let mut block_contents: Vec<String> = Vec::new();
  // 开发代码块起始行
  let mut block_begin_line: i32 = -1;
  // 开发代码块名称
  let mut block_name = String::new();
  // 当前行索引
  let mut line_number = 0u32;

  source_content.split("\n").for_each(|line| {
    println!("{:?}", line);

    match get_line_type(line, block_begin_line) {
      LineType::DevBegin => {
        block_begin_line = line_number as i32;
        block_name = get_dev_name(line);
        block_contents.push(String::from(line));
      },
      LineType::DevEnd => {
        block_contents.push(String::from(line));
        blocks.push(PearBlock {
          id: format!("{}", line_number),
          name: String::from(block_name.as_str()),
          line: block_begin_line as u32,
          file_path: String::from(source_file_path),
          total_dev_line: block_contents.len() as u32,
          content: block_contents.join("\n"),
        });
        block_begin_line = -1;
        block_name = String::from("");
        block_contents.clear();
      },
      LineType::DevCode => {
        block_contents.push(String::from(line));
      },
      LineType::Normal => {
        normal_code_source.push(String::from(line));
      }
    }

    line_number += 1;
  });

  let file_data = SourceFileData {
    file_path: String::from(source_file_path),
    contents: normal_code_source.join("\n"),
    dev_blocks: blocks,
  };

  Ok(file_data)
}

/// 获取当前行类型
pub fn get_line_type(line: &str, block_begin_line: i32) -> LineType {
  if is_dev_start(line) {
    return LineType::DevBegin;
  }

  if is_dev_end(line) {
    return LineType::DevEnd;
  }

  if block_begin_line >= 0 {
    return LineType::DevCode;
  }

  LineType::Normal
}

pub fn get_dev_name(line: &str) -> String {
  if !is_dev_start(line) {
    return String::from("");
  }
  let name = line.trim().replace(DEV_BEGIN_SIGN, "");
  name
}

pub fn is_dev_start(line: &str) -> bool {
  if line.contains(DEV_BEGIN_SIGN) {
    return true;
  }
  false
}

pub fn is_dev_end(line: &str) -> bool {
  if line.contains("// PEAR_END") {
    return true;
  }
  false
}

/// 代码行类型
pub enum LineType {
  // 开发开始
  DevBegin,
  // 开发代码结束
  DevEnd,
  // 开发代码块
  DevCode,
  // 正常代码
  Normal,
}

/// 开发代码分离后的文档内容
#[derive(Debug)]
pub struct SourceFileData {
  /// 源文件路径
  pub file_path: String,
  /// 源文件内容
  pub contents: String,
  /// 开发代码块数据
  pub dev_blocks: Vec<PearBlock>,
}

/// 代码块数据
#[derive(Debug, Deserialize, Serialize)]
pub struct PearBlock {
  /// 代码块ID
  pub id: String,
  /// 代码块自定义名称
  pub name: String,
  /// 对应的行
  pub line: u32,
  /// 对应的文件路径
  pub file_path: String,
  /// 开发代码块总行数
  pub total_dev_line: u32,
  /// 代码内容
  pub content: String,
}
```
