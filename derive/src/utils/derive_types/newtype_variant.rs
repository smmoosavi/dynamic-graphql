use super::{BaseVariant, TupleField};
use darling::{FromField, FromVariant};

pub struct NewtypeVariant<F: FromField = TupleField> {
    pub ident: syn::Ident,
    pub fields: F,
}

impl<F: FromField> FromVariant for NewtypeVariant<F> {
    fn from_variant(variant: &syn::Variant) -> darling::Result<Self> {
        let base = BaseVariant::<F>::from_variant(variant)?;
        if base.fields.len() > 1 {
            return Err(darling::Error::unsupported_shape("tuple variant").with_span(&base.ident));
        }
        let field = base.fields.into_iter().next().ok_or_else(|| {
            darling::Error::unsupported_shape("unit variant").with_span(&base.ident)
        })?;
        Ok(NewtypeVariant {
            ident: base.ident,
            fields: field,
        })
    }
}
