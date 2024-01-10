use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes, Debug, Clone)]
pub struct Oic {
    pub name: Option<syn::Lit>,
    pub data_type: Option<syn::Lit>,
    pub len: Option<syn::Lit>,
    pub default: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}
