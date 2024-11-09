#[allow(dead_code)]

pub(crate) mod attributes;
pub(crate) mod column;
pub(crate) mod list_filter;
pub(crate) mod utils;
pub(crate) mod error;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Column, attributes(oic))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match column::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

///
/// 为筛选条件增加 page page_size order_by order 等字段
/// 
#[proc_macro_attribute]
pub fn add_filter_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(input as DeriveInput);
    list_filter::add_filter_fields(&mut args).into()
}

#[proc_macro_derive(FilterParams, attributes(oic))]
pub fn derive_filter_params(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match list_filter::add_filter_methods(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

