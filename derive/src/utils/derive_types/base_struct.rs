use super::Base;
use crate::utils::with_context::SetContext;
use darling::{FromDeriveInput, FromField, FromGenerics};
use syn::DeriveInput;

#[derive(Debug, Clone)]
pub struct BaseStruct<F: FromField, G: FromGenerics = ()> {
    pub ident: syn::Ident,
    pub generics: G,
    pub data: darling::ast::Fields<F>,
}

impl<F, G> FromDeriveInput for BaseStruct<F, G>
where
    F: FromField,
    G: FromGenerics,
{
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let base: Base<(), F, G> = FromDeriveInput::from_derive_input(input)?;
        match base.data {
            darling::ast::Data::Enum(_) => {
                Err(darling::Error::unsupported_shape("enum").with_span(&base.ident))
            }
            darling::ast::Data::Struct(data) => Ok(BaseStruct {
                ident: base.ident,
                generics: base.generics,
                data,
            }),
        }
    }
}

impl<F, G> SetContext for BaseStruct<F, G>
where
    F: FromField + SetContext,
    G: FromGenerics,
{
    type Context = F::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.data.set_context(context);
    }
}
