#[allow(dead_code)]

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{
    Attribute,
    Meta,
    Token,
    punctuated::Punctuated,
};

pub(crate) fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("oicnp_api") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "oicnp_api".to_string(),
        };
        let name = Ident::new(&name, Span::call_site());
        quote!(#name)
    }
}

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