use darling::{FromDeriveInput, FromGenerics, FromVariant};
use syn::DeriveInput;

use crate::utils::with_context::SetContext;

use super::Base;

#[derive(Debug, Clone)]
pub struct BaseEnum<V: FromVariant, G: FromGenerics = ()> {
    pub ident: syn::Ident,
    pub generics: G,
    pub data: Vec<V>,
}

impl<V: FromVariant, G: FromGenerics> FromDeriveInput for BaseEnum<V, G> {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let base: Base<V, (), G> = FromDeriveInput::from_derive_input(input)?;
        match base.data {
            darling::ast::Data::Enum(data) => Ok(BaseEnum {
                ident: base.ident,
                generics: base.generics,
                data,
            }),
            darling::ast::Data::Struct(_) => {
                Err(darling::Error::unsupported_shape("struct").with_span(&base.ident))
            }
        }
    }
}

impl<V, G> SetContext for BaseEnum<V, G>
where
    V: FromVariant + SetContext,
    G: FromGenerics,
{
    type Context = V::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.data.set_context(context);
    }
}
