// use bae::FromAttributes;

#[derive(Default, Debug, Eq, PartialEq, Clone /* , FromAttributes */)]
pub struct Oic {
    pub name: Option<syn::Lit>,
    pub data_type: Option<syn::Lit>,
    pub len: Option<syn::Lit>,
    pub default: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}

impl Oic {
    pub fn from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut oic = Oic::default();
        for attr in attrs {
            println!("{:?}", attr);
            // if attr.path.is_ident("oic") {
            //     oic.name = attr.parse_meta()?.lit_str();
            // }
        }
        Ok(oic)
    }
}
