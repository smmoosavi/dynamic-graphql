use super::Base;
use darling::{FromDeriveInput, FromGenerics, FromVariant};
use syn::DeriveInput;

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
