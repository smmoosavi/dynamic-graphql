use darling::FromField;
use darling::util::Ignored;

use super::BaseField;
use crate::utils::with_context::SetContext;

#[derive(Debug, Clone)]
pub struct TupleField {
    pub ty: syn::Type,
}

impl FromField for TupleField {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let base = BaseField::from_field(field)?;
        if base.ident.is_some() {
            return Err(darling::Error::unsupported_shape("named field").with_span(&base.ty));
        }
        Ok(TupleField { ty: base.ty })
    }
}

impl SetContext for TupleField {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}
