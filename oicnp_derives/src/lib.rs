pub(crate) mod attributes;
pub(crate) mod column;
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
