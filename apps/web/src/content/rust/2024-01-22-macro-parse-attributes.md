---
title: 'Rust 宏开发之属性参数解析'
description: '开发过程宏时经常需要处理结构体或枚举体上的属性参数'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', '语法树', '属性参数']
---

## 场景

开发过程宏时经常需要处理结构体或枚举体上的属性参数，如下 Command 结构体的 `args` 字段有属性 `each = "arg"`,

```rust
#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
```

## AST

对应的语法树结构：

```ini
// Command 语法树
DeriveInput {
    // Command 结构体属性 当前没有
    attrs: [],
    vis: Visibility::Public(
        Pub,
    ),
    // 结构体名称
    ident: Ident {
        ident: "Command",
        span: #0 bytes(206..213),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Struct {
        struct_token: Struct,
        // Fields::Named字段集合
        fields: Fields::Named {
            brace_token: Brace,
            named: [
                // executable 字段
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    // 字段名 executable
                    ident: Some(
                        Ident {
                            ident: "executable",
                            span: #0 bytes(220..230),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    // 字段类型 String
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "String",
                                        span: #0 bytes(232..238),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                // args 字段
                Field {
                    // 字段指定的属性参数集合
                    attrs: [
                        Attribute {
                            pound_token: Pound,
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::List {
                                // 属性名称 builder
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "builder",
                                                span: #0 bytes(246..253),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                delimiter: MacroDelimiter::Paren(
                                    Paren,
                                ),
                                tokens: TokenStream [
                                    // 第一个属性 key = each
                                    Ident {
                                        ident: "each",
                                        span: #0 bytes(254..258),
                                    },
                                    Punct {
                                        ch: '=',
                                        spacing: Alone,
                                        span: #0 bytes(259..260),
                                    },
                                    // 第一个属性 value = arg
                                    Literal {
                                        kind: Str,
                                        symbol: "arg",
                                        suffix: None,
                                        span: #0 bytes(261..266),
                                    },
                                ],
                            },
                        },
                    ],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    // 字段名 args
                    ident: Some(
                        Ident {
                            ident: "args",
                            span: #0 bytes(273..277),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    // 字段类型 Vec<String>
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "Vec",
                                        span: #0 bytes(279..282),
                                    },
                                    arguments: PathArguments::AngleBracketed {
                                        colon2_token: None,
                                        lt_token: Lt,
                                        args: [
                                            GenericArgument::Type(
                                                Type::Path {
                                                    qself: None,
                                                    path: Path {
                                                        leading_colon: None,
                                                        segments: [
                                                            PathSegment {
                                                                ident: Ident {
                                                                    ident: "String",
                                                                    span: #0 bytes(283..289),
                                                                },
                                                                arguments: PathArguments::None,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                        ],
                                        gt_token: Gt,
                                    },
                                },
                            ],
                        },
                    },
                },
    // 其它字段省略
```

## 宏入口

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenSteam {
  let input = parse_macro_input(input as DeriveInput);
  // 从 data中获取 fields 字段数据集合
  let fields = ...;

  let setter_fns = fields.iter().map(|f| {
    // 无属性参数时功能处理
    if f.attrs.is_empty() { return quote!{}; }

    // 遍历属性参数
    for attr in f.attrs.iter() {
      // parse_builder_attr 解析当前的属性数据
      let arguments = parse_builder_attr(attr);
      // 根据属性参数再处理其它处理
    }
  });
}
```

## 属性解析实现

```rust
/// builder 属性参数数据类型
struct BuilderAttribute {
  /// 属性Key
  pub key : String,
  /// 属性值
  pub value: Option<String>,
  /// 属性meta path, 用于错误提示位置显示
  /// 类似：
  ///   --> main.rs:14:7
  ///    |
  ///  14|    #[builder(each = "arg")]
  ///    |      ^^^^^^^^^^^^^^^^^^^^
  pub meta: Meta,
}

/// 属性参数解析
/// #[builder(each = "env")]
/// 转为
/// [BuilderAttribute { key: "each", value: Some("env"), meta: syn::Meta }]
fn parse_builder_attr(attr: &syn::Attribute) -> Vec<BuilderAttribute> {
    let mut arguments = Vec::new();

    if attr.path().is_ident("builder") {
        // parse_args_with 参考:
        // https://docs.rs/syn/2.0.48/syn/struct.Attribute.html#alternatives
        // nested 类型 Vec<Meta::NameValue>
        if let Ok(nested) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
            for meta in &nested {
                if let Some(ident) = meta.path().get_ident() {
                    let key = ident.to_string();
                    let mut value: Option<String> = None;

                    if let Meta::NameValue(name_value) = meta {
                        if let Expr::Lit(ref expr_lit) = name_value.value {
                            value = match expr_lit.lit {
                                Lit::Str(ref lit_str) => Some(lit_str.value()),
                                Lit::Bool(ref lit_bool) => Some(lit_bool.value.to_string()),
                                Lit::Int(ref lit_int) => Some(lit_int.base10_digits().to_owned()),
                                _ => None,
                            }
                        }
                    };

                    arguments.push(BuilderAttribute {
                        key,
                        value,
                        meta: attr.meta.clone(),
                    });
                }
            }
        }
    }

    arguments
}
```

## 解析结果

```ini
[
    BuilderAttribute {
        // 属性键
        key: "each",
        // 属性值
        value: Some(
            "arg",
        ),
        // Meta
        meta: Meta::List {
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            ident: "builder",
                            span: #0 bytes(246..253),
                        },
                        arguments: PathArguments::None,
                    },
                ],
            },
            delimiter: MacroDelimiter::Paren(
                Paren,
            ),
            tokens: TokenStream [
                Ident {
                    ident: "each",
                    span: #0 bytes(254..258),
                },
                Punct {
                    ch: '=',
                    spacing: Alone,
                    span: #0 bytes(259..260),
                },
                Literal {
                    kind: Str,
                    symbol: "arg",
                    suffix: None,
                    span: #0 bytes(261..266),
                },
            ],
        },
    },
]
```
