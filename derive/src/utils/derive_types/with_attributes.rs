use darling::{FromAttributes, FromDeriveInput, FromField, FromVariant};
use std::ops::Deref;

pub struct WithAttributes<A: FromAttributes, D> {
    pub attrs: A,
    pub inner: D,
}

// deref
impl<A: FromAttributes, D> Deref for WithAttributes<A, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<A: FromAttributes, D: FromDeriveInput> FromDeriveInput for WithAttributes<A, D> {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let attrs = A::from_attributes(&input.attrs)?;
        let data = D::from_derive_input(input)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}

impl<A: FromAttributes, D: FromField> FromField for WithAttributes<A, D> {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let attrs = A::from_attributes(&field.attrs)?;
        let data = D::from_field(field)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}

impl<A: FromAttributes, D: FromVariant> FromVariant for WithAttributes<A, D> {
    fn from_variant(variant: &syn::Variant) -> darling::Result<Self> {
        let attrs = A::from_attributes(&variant.attrs)?;
        let data = D::from_variant(variant)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}
