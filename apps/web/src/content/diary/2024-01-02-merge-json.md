---
title: 'Rust JSON操作之合并两个JSON对象'
description: 'serde_json 数据常用操作，合并两个JSON Object对象'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'json', '合并']
---

serde_json Object 数据合并操作

```rust
use serde_json::Value;

/// 合并两个JSON对象
fn merge_json(obj_a: Value, obj_b: Value) -> Value {
  let map_a = obj_a.as_object();
  let map_b = obj_b.as_object();

  // 任意参数转为 Object 失败直接返回
  if map_a.is_none() || map_b.is_none() {
    return Value::Null;
  }

  let map_a = map_a.unwrap();
  let map_b = map_b.unwrap();

  let mut data = map_a.clone();

  for (key, value) in map_b.iter() {
    data.insert(key.into(), value.clone());
  }

  Value::Object(data)
}
```

示例：

```rust
use serde_json::json;

fn main() {
  let a = json!({
    "id": 1,
    "name": "Alex",
    "age": 18,
  });

  let b = merge_json(a, json!({
    "name": "bob",
  }));

  println!("合并后： {:?}", b);
}
```

输出结果类似：

```
合并后：Object {"age": Number(18), "id": Number(1), "name": String("bob")}
```
