use super::{BaseVariant, TupleField};
use crate::utils::with_context::SetContext;
use darling::{FromField, FromVariant};

#[derive(Debug, Clone)]
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

impl<F> SetContext for NewtypeVariant<F>
where
    F: FromField + SetContext,
{
    type Context = F::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.fields.set_context(context);
    }
}
