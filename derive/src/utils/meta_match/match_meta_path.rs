use crate::utils::meta_match::{MatchNestedMeta, MatchPath};
use std::ops::Deref;
use syn::spanned::Spanned;

pub struct MatchMetaPath<P: MatchPath = syn::Path>(pub P);

impl<P: MatchPath> Deref for MatchMetaPath<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Spanned for MatchMetaPath {
    fn span(&self) -> proc_macro2::Span {
        self.0.span()
    }
}

impl<P: MatchPath> MatchNestedMeta for MatchMetaPath<P> {
    fn match_nested_meta(meta: &syn::NestedMeta) -> Option<darling::Result<Self>> {
        match meta {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                P::match_path(path).map(|p| Ok(MatchMetaPath(p?)))
            }
            _ => None,
        }
    }
}
