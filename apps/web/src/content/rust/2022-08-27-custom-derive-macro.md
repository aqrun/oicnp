---
title: 'Rust 过程宏之derive派生宏实战入门'
description: '过程宏（Procedure Macro）是Rust中的一种特殊形式的宏，提供比普通宏更强大的功能。过程宏主要分三类：派生宏（Derive macro）、属性宏（Attribute macro）、函数式宏'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'blockchain', 'crypto', 'p2p']
---

## 需求:

获取枚举类型附加信息，如备注说明，传统作法如下：

实现 comment 方法根据匹配值返回对应字符串

```rust
enum Robot {
      Alex,
      Bob
}

impl Robot {
      fn comment(&self) -> &'static str {
            match self {
                  Robot::Alex => "这是Alex",
                  Robot::Bob => "这是Bob",
            }
      }
}

// test
println!("alex comment: {}", Robot::Alex.comment()); // alex comment: 这是Alex
```

传统作法有一个问题，如果有很多枚举，就需要各自实现对应的 comment 方法实现，很麻烦，是否可以简化呢？

可以的！

其中 comment 方法的实现就属于模板代码。这种重复性的工作就可以考虑使用宏，最终简化如下：

除了 comment 也可以附加 其它信息如 姓名、年龄等

```rust
#[derive(HelloMacro)]
enum Robot {
    #[oic_column(name = "alex_name", age = 22, comment = "这是Alex")]
    Alex,
    #[oic_column(name = "bob_name", age = 50, comment = "这是Bob")]
    Bob,
}

// 测试可以获取一样的结果
println!("alex comment: {}", Robot::Alex.comment()); // alex comment: 这是Alex
```

## 什么是过程宏

> 过程宏中文文档参考： https://zjp-cn.github.io/rust-note/proc/quote.html

`过程宏（Procedure Macro）` 是 Rust 中的一种特殊形式的宏，提供比普通宏更强大的功能。过程宏主要分三类：

- **派生宏（Derive macro）**：用于结构体（struct）、枚举（enum）、联合（union）类型，可为其实现函数或特征（Trait）。
- **属性宏（Attribute macro）**：用在结构体、字段、函数等地方，为其指定属性等功能。如标准库中的#[inline]、#[derive(...)]等都是属性宏。
- **函数式宏（Function-like macro）**：用法与普通的规则宏类似，但功能更加强大，可实现任意语法树层面的转换功能。

### 1.派生宏示例：

```rust
#[proc_macro_derive(Builder)]
fn derive_builder(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}
```

其使用方法如下：

```rust
#[derive(Builder)]
struct Command {
    // ...
}
```

### 2.属性宏示例

```rust
#[proc_macro_attribute]
fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}
```

其使用方法如下：

```rust
#[sorted]
enum Letter {
    A,
    B,
    C,
    // ...
}
```

### 3.函数式宏示例

```rust
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}
```

其使用方法如下：

```rust
seq! { n in 0..10 {
    /* ... */
}}
```

## 功能实现

本例主要使用派生宏实现

项目创建可以参数[示例仓库 https://github.com/imbolc/rust-derive-macro-guide](https://github.com/imbolc/rust-derive-macro-guide)

### 第一步 创建测试项目

测试项目: my-macro-test
宏项目： my-derives

项目结构最终如下：

```
my-macro-test/
      my-derives/             // 子项目
            src/
                  attributes.rs
                  lib.rs
            Cargo.toml
      src/
            main.rs             // 主项目入口
      Cargo.toml
```

文件 my-macro-test/Cargo.toml

```toml
# my-macro-test/Cargo.toml
[package]
name = "my-macro-test"
version = "0.1.0"
edition = "2021"

[dependencies]
# 根据路径指定子项目
my_derives = { path = "my-derives" }
```

文件 my-macro-test/my-derives/Cargo.toml

```toml
[package]
# my-macro-test/my-derives/Cargo.toml
name = "my-derives"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "^1", features = ["full"] }
quote = "^1"
proc-macro2 = "^1"
bae = "^0"
```

重要的第三方库：

- [quote](https://docs.rs/quote) 把 Rust 语法树的数据结构转化为源代码的标记 (tokens)
- [syn](https://docs.rs/syn) 主要是一个解析库，用于把 Rust 标记流解析为 Rust 源代码的语法树
- [bae](https://docs.rs/bae) 简化属性数据的处理

### 第二步 自定义属性定义

文件： my-derives/attributes.rs

```rust
// my-derives/attributes.rs
use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes, Debug)]
pub struct OicColumn {
    pub name: Option<syn::Lit>,
    pub age: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}
```

### 第三步 HelloMacro 派生宏实现

文件： my-derives/lib.rs

```rust
mod attributes;

use proc_macro::{self, TokenStream};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};
use attributes::{OicColumn};

// HelloMacro 定义
#[proc_macro_derive(HelloMacro, attributes(oic_column))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    // ident 当前枚举名称
    let DeriveInput { ident, .. } = input;

    let mut comment_arms = Vec::new();

    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            // 当前枚举项名称如 Alex, Box
            let ident_item = &variant.ident;
            // 根据属性值转为 OicColumn 定义的结构化数据
            if let Ok(column) = OicColumn::from_attributes(&variant.attrs) {
                // 获取属性中的comment信息
                let msg: &syn::Lit = &column.comment.unwrap();

                // 生成 match 匹配项 Robot::Alex => "msg"
                comment_arms.push(quote! ( #ident::#ident_item => #msg ));
            } else {
                comment_arms.push(quote! ( #ident::#ident_item => "" ));
            }
        }
    }

    if comment_arms.is_empty() {
        comment_arms.push(quote! ( _ => "" ));
    }

    // 实现 comment 方法
    let output = quote! {
        impl #ident {
            fn comment(&self) -> &'static str {
                match self {
                    #(#comment_arms),*
                }
            }
        }
    };
    output.into()
}
```

`#(#comment_arms),*` 为数据解构语法，具体参考：https://docs.rs/quote/1.0.21/quote/macro.quote.html

### 第四步 测试代码实现

文件： my-macro-test/main.rs

```rust
use my_derives::HelloMacro;

#[derive(HelloMacro)]
enum Robot {
    #[oic_column(name = "alex_name", age = 22, comment = "这是Alex")]
    Alex,
    #[oic_column(name = "bob_name", age = 50, comment = "这是Bob")]
    Bob,
    Apple,
}

fn main() {
    // test comment: Alex: "这是Alex", ----- Bob: "这是Bob"
    println!("test comment: Alex: {:?}, ----- Bob: {:?}", Robot::Alex.comment(), Robot::Bob.comment());
    // test comment apple: ""
    println!("test comment apple: {:?}", Robot::Apple.comment());
}
```

cargo run 运行测试输出结果如注释部分。
