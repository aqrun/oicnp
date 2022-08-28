use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes, Debug)]
pub struct OicColumn {
    pub name: Option<syn::Lit>,
    pub age: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}
