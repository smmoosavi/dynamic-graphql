#[derive(Debug, Clone)]
pub enum Deprecation {
    NoDeprecated,
    Deprecated { reason: Option<String> },
}

impl Default for Deprecation {
    fn default() -> Self {
        Deprecation::NoDeprecated
    }
}

impl darling::FromMeta for Deprecation {
    fn from_word() -> darling::Result<Self> {
        Ok(Deprecation::Deprecated { reason: None })
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            syn::Lit::Bool(syn::LitBool { value: true, .. }) => {
                Ok(Deprecation::Deprecated { reason: None })
            }
            syn::Lit::Bool(syn::LitBool { value: false, .. }) => Ok(Deprecation::NoDeprecated),
            syn::Lit::Str(str) => Ok(Deprecation::Deprecated {
                reason: Some(str.value()),
            }),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}
