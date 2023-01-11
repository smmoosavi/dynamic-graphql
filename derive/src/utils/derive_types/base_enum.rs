use super::Base;
use darling::{FromDeriveInput, FromVariant};
use syn::DeriveInput;

pub struct BaseEnum<V: FromVariant> {
    pub ident: syn::Ident,
    pub data: Vec<V>,
}

impl<V: FromVariant> FromDeriveInput for BaseEnum<V> {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let base: Base<V, ()> = FromDeriveInput::from_derive_input(input)?;
        match base.data {
            darling::ast::Data::Enum(data) => Ok(BaseEnum {
                ident: base.ident,
                data,
            }),
            darling::ast::Data::Struct(_) => {
                Err(darling::Error::unsupported_shape("struct").with_span(&base.ident))
            }
        }
    }
}
