use darling::ast::NestedMeta;
use std::ops::Deref;

use crate::utils::meta_match::MatchNestedMeta;
use crate::utils::meta_match::MatchString;

pub struct MatchLitStr<S: MatchString = String>(pub S, pub proc_macro2::Span);

impl<S: MatchString> Deref for MatchLitStr<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S: MatchString> MatchNestedMeta for MatchLitStr<S> {
    fn match_nested_meta(meta: &NestedMeta) -> Option<darling::Result<Self>> {
        match meta {
            NestedMeta::Lit(syn::Lit::Str(string)) => {
                S::match_string(string.value().as_str()).map(|s| Ok(MatchLitStr(s?, string.span())))
            }
            _ => None,
        }
    }
}
