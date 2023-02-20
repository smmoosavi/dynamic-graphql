use darling::util::Ignored;
use darling::FromDeriveInput;

use super::BaseStruct;
use crate::utils::with_context::SetContext;

#[derive(Debug, Clone)]
pub struct UnitStruct {
    pub ident: syn::Ident,
}

impl FromDeriveInput for UnitStruct {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let base: BaseStruct<(), ()> = FromDeriveInput::from_derive_input(input)?;
        if !base.data.fields.is_empty() {
            return Err(darling::Error::unsupported_shape("non unit struct").with_span(&base.ident));
        }
        Ok(UnitStruct { ident: base.ident })
    }
}

impl SetContext for UnitStruct {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}
