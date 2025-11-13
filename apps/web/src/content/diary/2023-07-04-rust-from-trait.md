---
title: Rust From 特征
description: '用于将输入数据转为当前类型，和Into功能正好相反。通常优先实现 From 而不是Into，因为标准库会在实现From时自动实现Into特征'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'from', 'trait']
---

## 特征 `std::convert::From`

```rust
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
```

用于将输入数据转为当前类型，和 [Into](https://doc.rust-lang.org/std/convert/trait.Into.html)功能正好相反。

通常优先实现 From 而不是 Into，因为标准库会在实现 From 时自动实现 Into 特征。

在泛型函数上指定特征边界时，优先使用 Into。这样直接实现 Into 的类型也可以用作参数。

From 在执行错误处理时非常有用。函数返回类型通常如： `Result<T, E>`。From 特征允许函数返回封装了个多个
错误类型的单一错误类型，从而简化错误处理。

## 标准库 String 实现 From 特征示例

将 &str 类型转为 String 类型

```rust
let my_string = String::from("hello");
```

## Graphql 中类型转换

从数据库获取的数据是 `CoreNodeBody` 类型，Graphql Object 是 `NodeBody`

```rust
#[derive(Clone, Debug)]
pub struct NodeBody {
    pub nid: String,
    pub summary: String,
    pub summary_format: String,
    pub body: String,
    pub body_format: String,
}

#[Object]
impl NodeBody {
    async fn nid(&self) -> &str {
        self.nid.as_str()
    }
    async fn summary(&self) -> &str {
        self.summary.as_str()
    }
    async fn body(&self) -> &str {
        self.body.as_str()
    }
    async fn body_format(&self) -> &str {
        self.body_format.as_str()
    }
}

// 实现 From 特征
impl From<&CoreNodeBody> for NodeBody {
    fn from(nb: &CoreNodeBody) -> Self {
        Self {
            nid: String::from(&nb.nid),
            summary: String::from(&nb.summary),
            summary_format: String::from(&nb.summary_format),
            body: String::from(&nb.body),
            body_format: String::from(&nb.body_format),
        }
    }
}
```

## 应用示例

```rust
// graphsql 查询接口
async fn body_query() -> Result<NodeBody> {
	let core_node_body: CoreNodeBody = find_node_body_from_db();
	// 类型转换
	let res: NodeBody = NodeBody::from(&core_node_body);
	Ok(res)
}
```
