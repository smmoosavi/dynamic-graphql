use super::BaseVariant;
use darling::FromVariant;

pub struct UnitVariant {
    pub ident: syn::Ident,
}

impl FromVariant for UnitVariant {
    fn from_variant(variant: &syn::Variant) -> darling::Result<Self> {
        let base = BaseVariant::<()>::from_variant(variant)?;
        if !base.fields.is_empty() {
            return Err(
                darling::Error::unsupported_shape("non unit variant").with_span(&base.ident)
            );
        }
        Ok(UnitVariant { ident: base.ident })
    }
}
