---
title: 'Rust 语法树解析Option内部类型'
description: '使用Syn库解析结构体Option字段具体的内部类型'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', '语法树', 'syn']
---

## 场景：

如下 Command 结构体

```rust
#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
```

解析语法树判断 current_dir 字段是否为 Option 类型， 如果是 Option 获取内部的 String 类型。

## 打印语法树

```rust
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);

    TokenStream::new()
}
```

字段 current_dir 对应语法树输出结果如下：

```rust
Field {
    attrs: [],
    vis: Visibility::Inherited,
    mutability: FieldMutability::None,
    ident: Some(
        Ident {
            ident: "current_dir",
            span: #0 bytes(289..300),
        },
    ),
    colon_token: Some(
        Colon,
    ),
    ty: Type::Path {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [
                PathSegment {
                    ident: Ident {
                        ident: "Option",
                        span: #0 bytes(302..308),
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
                                                    span: #0 bytes(309..315),
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
```

## 解析当前字段的 Type 类型数据

```rust
// 要返回的类型数据
struct InnerTypeData {
    // 是否为 Option 类型
    is_option: bool,
    // 具体内部类型
    ty: Type,
}

/// 获取字段类型的内部类型
/// ty: 当前字段的Type数据
fn get_inner_type(ty: &Type) -> Result<InnerTypeData, Box<dyn std::error::Error>> {
    // 默认值
    let mut inner_ty_data = InnerTypeData {
        is_option: false,
        ty: ty.clone(),
    };

    if let syn::Type::Path(ref type_path) = ty {
        // 外层 segments 必须有值
        if type_path.path.segments.len() <= 0 {
            return Err("Segments empty".into());
        }

        // 外层 segments 第一条数据
        let path_segments = type_path.path.segments.first().unwrap();

        // 外层类型是 Option
        if path_segments.ident.eq("Option") {
            inner_ty_data.is_option = true;
        }

        // 不是 option 类型直接返回
        if !inner_ty_data.is_option {
            return Ok(inner_ty_data);
        }

        // 是 Option 再解构出 Option 内层类型
        if let syn::PathArguments::AngleBracketed(ref path_args) = path_segments.arguments {
            if path_args.args.len() <= 0 {
                return Err("Args 数据为空".into());
            }

            // 获取到内部类型
            if let syn::GenericArgument::Type(ref arg_type) = path_args.args.first().unwrap() {
                inner_ty_data.ty = arg_type.clone();
            }
        }
    }

    Ok(inner_ty_data)
}
```

## 示例：根据 InnerTypeData 生成 setters 方法

```rust
// setter 设置值参数
// 遍历字段列表
let setter_fns = fields.iter().map(|f| {
    // 当前字段名称
    let name = &f.ident;
    // 当前字段类型
    let ty = &f.ty;

    // 内部类型数据
    let inner_type_data = get_inner_type(ty).unwrap();
    // 具体的内部类型
    let inner_type = inner_type_data.ty;

    quote! {
        pub fn #name(&mut self, #name: #inner_type) -> &mut Self {
            self.#name = Some(#name);
            self
        }
    }
});

quote! {
  impl #builder_name {
      // 建造者 setters 插值遍历
      #(#setter_fns)*
  }
}
```

cargo expand 输出结果：

```rust
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
    }
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
    }
    pub fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = Some(env);
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
    }
}
```
