use super::BaseField;
use crate::utils::with_context::SetContext;
use darling::util::Ignored;
use darling::FromField;
use syn::Field;

#[derive(Debug, Clone)]
pub struct NamedField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

impl FromField for NamedField {
    fn from_field(field: &Field) -> darling::Result<Self> {
        let base = BaseField::from_field(field)?;
        if base.ident.is_none() {
            return Err(darling::Error::unsupported_shape("unnamed field").with_span(&base.ty));
        }
        Ok(NamedField {
            ident: base.ident.unwrap(),
            ty: base.ty,
        })
    }
}

impl SetContext for NamedField {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}
