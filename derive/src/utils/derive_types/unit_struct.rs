use super::BaseStruct;
use darling::FromDeriveInput;

pub struct UnitStruct {
    pub ident: syn::Ident,
}

impl FromDeriveInput for UnitStruct {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let base: BaseStruct<()> = FromDeriveInput::from_derive_input(input)?;
        if !base.data.fields.is_empty() {
            return Err(darling::Error::unsupported_shape("non unit struct").with_span(&base.ident));
        }
        Ok(UnitStruct { ident: base.ident })
    }
}
