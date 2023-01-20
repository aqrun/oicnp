use proc_macro2::{TokenStream, Literal};
use quote::quote;
use syn::{DeriveInput, Lit};

use crate::{
    error::GeneratorResult,
};
use crate::attributes::OicColumn;

pub(crate) fn generate(input: DeriveInput) -> GeneratorResult<TokenStream> {
    // ident 当前枚举名称
    let DeriveInput { ident, .. } = input;

    let mut comment_arms = Vec::new();

    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            // 当前枚举项名称如 Alex, Box
            let ident_item = &variant.ident;
            // 根据属性值转为 OicColumn 定义的结构化数据
            if let Ok(column) = OicColumn::from_attributes(&variant.attrs) {
                let default_lit_str = Lit::new(Literal::string(""));

                let comment = &column
                    .comment
                    .unwrap_or(default_lit_str);

                // 生成 match 匹配项 Robot::Alex => "msg"
                comment_arms.push(quote! ( #ident::#ident_item => #comment ));
            } else {
                comment_arms.push(quote! ( #ident::#ident_item => "" ));
            }
        }
    }

    if comment_arms.is_empty() {
        comment_arms.push(quote! ( _ => "" ));
    }

    let expanded = quote! {
        impl #ident {
            fn comment(&self) -> &'static str {
                match self {
                    #(#comment_arms),*
                }
            }
        }
    };
    Ok(expanded)
}