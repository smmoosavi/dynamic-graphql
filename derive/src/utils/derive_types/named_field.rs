use super::BaseField;
use darling::FromField;
use syn::Field;

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
