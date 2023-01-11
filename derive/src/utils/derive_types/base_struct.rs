use super::Base;
use darling::{FromDeriveInput, FromField, FromGenerics};
use syn::DeriveInput;

pub struct BaseStruct<F: FromField, G: FromGenerics = ()> {
    pub ident: syn::Ident,
    pub generics: G,
    pub data: darling::ast::Fields<F>,
}

impl<F: FromField, G: FromGenerics> FromDeriveInput for BaseStruct<F, G> {
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
