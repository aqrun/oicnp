use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse::Parser};
use crate::utils::parse_derive_attributes;

use crate::error::GeneratorResult;

///
/// 添加筛选参数相关操作
/// 
pub(crate) fn add_filter_methods(input: DeriveInput) -> GeneratorResult<TokenStream> {
    // ident 当前枚举名称
    let DeriveInput { ident, .. } = input;

    let expanded = quote! {
        impl #ident {
            /// 页码
            pub fn get_page(&self) -> u64 {
                self.page.unwrap_or(1)
            }

            /// 每页数据个数
            pub fn get_page_size(&self) -> u64 {
                self.page_size.unwrap_or(10)
            }

            /// 排序字段名
            pub fn get_order_by(&self) -> String {
                if let Some(ref order_by) = self.order {
                    return String::from(order_by);
                }

                String::from("")
            }
            
            /// 排序方式
            pub fn get_order(&self) -> sea_orm::Order {
                if let Some(ref order) = self.order {
                    if order.eq("desc") {
                        return sea_orm::Order::Desc;
                    }
                }

                sea_orm::Order::Asc
            }
        }
    };
    Ok(expanded)
}

///
/// 添加筛选相关公共参数
/// 
/// page: Option<u64>
/// page_size: Option<u64>
/// order_by: Option<String>
/// order: Option<String>
/// 
pub(crate) fn add_filter_fields(input: &mut DeriveInput) -> TokenStream {
    // let struct_name = input.ident.to_string();
    // 获取全部 derive 属性列表
    let attr_list = parse_derive_attributes(input.attrs.as_slice());
    let has_deserialize = attr_list.contains(&String::from("Deserialize"));

    let expanded = match input.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    let named_field_parser = syn::Field::parse_named;

                    let page_size_token: TokenStream = if has_deserialize {
                        quote! {
                            #[serde(rename(deserialize = "pageSize"))]
                            pub page_size: std::option::Option<u64>
                        }
                    } else {
                        quote! {
                            pub page_size: std::option::Option<u64>
                        }
                    };
                    let order_by_token = if has_deserialize {
                        quote! {
                            #[serde(rename(deserialize = "orderBy"))]
                            pub order_by: std::option::Option<String>
                        }
                    } else {
                        quote! {
                            pub order_by: std::option::Option<String>
                        }
                    };

                    let page_field = named_field_parser
                        .parse2(quote! { pub page: std::option::Option<u64> }).unwrap();
                    let page_size_field = named_field_parser
                        .parse2(page_size_token).unwrap();
                    let order_by_field = named_field_parser
                        .parse2(order_by_token).unwrap();
                    let order_field = named_field_parser
                        .parse2(quote! { pub order: std::option::Option<String> }).unwrap();

                    fields
                        .named
                        .push(page_field);
                    fields
                        .named
                        .push(page_size_field);
                    fields
                        .named
                        .push(order_by_field);
                    fields
                        .named
                        .push(order_field);
                }   
                _ => { }
            };
            
            quote! { #input }
        },
        _ => panic!("`add_filter_fields` 只实现了结构体操作"),
    };

    expanded
}