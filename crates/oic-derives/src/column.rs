use proc_macro2::{TokenStream, Literal};
use quote::quote;
use syn::{DeriveInput, Lit};
use heck::ToSnakeCase;

use crate::error::GeneratorResult;
use crate::attributes::Oic as OicAttributes;

pub(crate) fn generate(input: DeriveInput) -> GeneratorResult<TokenStream> {
    // ident 当前枚举名称
    let DeriveInput { ident, .. } = input;

    let mut valid_table_name = ident.to_string().to_snake_case();

    let default_lit_str = Lit::new(Literal::string(""));
    let default_lit_i32 = Lit::new(Literal::i32_unsuffixed(0));

    let mut name_arms = Vec::new();
    let mut data_type_arms = Vec::new();
    let mut len_arms = Vec::new();
    let mut default_value_arms = Vec::new();
    let mut comment_arms = Vec::new();

    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            // 当前枚举项名称如 Alex, Box
            let ident_item = &variant.ident;

            // 存在 table 字段
            let is_table_field = ident_item.eq("Table");

            // 根据属性值转为 OicAttributes 定义的结构化数据
            // Oic 结体体名需要和属性名对应
            if let Ok(column) = OicAttributes::from_attributes(&variant.attrs) {
                let name = &column.name.unwrap_or(default_lit_str.clone());             
                let comment = &column.comment.unwrap_or(default_lit_str.clone());
                let data_type = &column.data_type.unwrap_or(default_lit_str.clone());
                let len = &column.len.unwrap_or(default_lit_i32.clone());
                let default_value_data = &column.default.unwrap_or(default_lit_str.clone());
                let default_value = match default_value_data {
                    syn::Lit::Str(item) => {
                        Lit::new(Literal::string(item.value().as_str()))
                    },
                    syn::Lit::Int(item) => {
                        Lit::new(Literal::string(item.base10_digits()))
                    },
                    _ => default_lit_str.clone(),
                };

                let mut name_arm = quote! ( #ident::#ident_item => #name );

                // Table  有指定name 属性
                if is_table_field {
                    if let syn::Lit::Str(item) = name {
                        valid_table_name = String::from(item.value().as_str());
                        name_arm = quote! ( #ident::#ident_item => #valid_table_name );
                    }
                }

                name_arms.push(name_arm);
                data_type_arms.push(quote! ( #ident::#ident_item => #data_type ));
                len_arms.push(quote! ( #ident::#ident_item => #len ));
                default_value_arms.push(quote! ( #ident::#ident_item => #default_value ));
                // 生成 match 匹配项 Robot::Alex => "msg"
                comment_arms.push(quote! ( #ident::#ident_item => #comment ));
            } else {
                comment_arms.push(quote! ( #ident::#ident_item => "" ));
            }
        }
    }

    name_arms.push(quote! ( _ => "" ));
    data_type_arms.push(quote! ( _ => "" ));
    len_arms.push(quote! ( _ => 0 ));
    default_value_arms.push(quote! ( _ => "" ));
    comment_arms.push(quote! ( _ => "" ));

    let expanded = quote! {
        impl #ident {
            pub fn table_name(prefix: &str) -> sea_orm::sea_query::Alias {
                let name = format!("{}{}", prefix, #valid_table_name);
                sea_orm::sea_query::Alias::new(name.as_str())
            }

            ///  字段名称
            pub fn name(&self) -> &'static str {
                match self {
                    #(#name_arms),*
                }
            }

            /// 字段类型
            pub fn data_type(&self) -> &'static str {
                match self {
                    #(#data_type_arms),*
                }
            }

            /// 字段大小
            pub fn len(&self) -> i32 {
                match self {
                    #(#len_arms),*
                }
            }

            /// 字段默认值
            pub fn default_value(&self) -> &'static str {
                match self {
                    #(#default_value_arms),*
                }
            }

            /// 字段注释
            pub fn comment(&self) -> &'static str {
                match self {
                    #(#comment_arms),*
                }
            }
        }
    };
    Ok(expanded)
}
