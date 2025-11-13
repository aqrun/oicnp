---
title: 'Rust 宏开发之读取结构体属性宏列表'
description: 'Rust 宏开发，使用 `sync` 库解析结构体宏列表转为字符串数组'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'macro']
---

# Rust 宏开发之读取结构体属性宏列表

## 需求

解析结构体 `ExampleNoteFilters` 判断是否指定了 `Deserialize` 属性宏

```rust
#[add_filter_fields]
#[derive(FilterParams, Deserialize)]
pub struct ExampleNoteFilters {
    pub id: Option<i64>,
    pub title: Option<String>,
}
```

## 功能实现

```rust
use quote::quote;
use syn::{Attribute,Meta,Token, punctuated::Punctuated};

///
/// 解析结构体属性宏转为字符串数组
/// 
/// #[derive(Debug, Display)]
/// struct Abc {
/// }
/// 
/// parse_derive_attributes(input.attrs)
/// 
pub(crate) fn parse_derive_attributes(attrs: &[Attribute]) -> Vec<String> {
    let mut list = Vec::new();

    let attrs = attrs.iter().filter(|attr| {
        attr.path().is_ident("derive")
    }).collect::<Vec<&Attribute>>();

    for attr in attrs {
        if let Ok(nested) = attr.parse_args_with(
            Punctuated::<Meta, Token![,]>::parse_terminated
        ) {
            for meta in nested.iter() {
                if let Some(ident) = meta.path().get_ident() {
                    let attribute_name = ident.to_string();

                    list.push(attribute_name);
                }
            }
        }
    }

    list
}
```

## 测试

执行 `cargo expand` 操作


```shell
cargo expand --example filter-params-derive
```

输出类似：

```rust
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oic_derives::{FilterParams, add_filter_fields};
use serde::Deserialize;

pub struct ExampleNoteFilters {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub page: std::option::Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: std::option::Option<u64>,
    #[serde(rename(deserialize = "orderBy"))]
    pub order_by: std::option::Option<String>,
    pub order: std::option::Option<String>,
}
```

`FilterParams` 宏的功能是: 如果存在 `Deserialize` 属性就会为 `page_size` `order_by` 
指定 `rename` 重命名参数

