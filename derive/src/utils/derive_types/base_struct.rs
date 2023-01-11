use super::Base;
use darling::{FromDeriveInput, FromField};
use syn::DeriveInput;

pub struct BaseStruct<F: FromField> {
    pub ident: syn::Ident,
    pub data: darling::ast::Fields<F>,
}

impl<F: FromField> FromDeriveInput for BaseStruct<F> {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let base: Base<(), F> = FromDeriveInput::from_derive_input(input)?;
        match base.data {
            darling::ast::Data::Enum(_) => {
                Err(darling::Error::unsupported_shape("enum").with_span(&base.ident))
            }
            darling::ast::Data::Struct(data) => Ok(BaseStruct {
                ident: base.ident,
                data,
            }),
        }
    }
}
