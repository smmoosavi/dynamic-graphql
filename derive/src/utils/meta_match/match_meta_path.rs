use darling::ast::NestedMeta;
use std::ops::Deref;

use crate::utils::meta_match::MatchNestedMeta;
use crate::utils::meta_match::MatchPath;

pub struct MatchMetaPath<P: MatchPath = syn::Path>(pub P);

impl<P: MatchPath> Deref for MatchMetaPath<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: MatchPath> MatchNestedMeta for MatchMetaPath<P> {
    fn match_nested_meta(meta: &NestedMeta) -> Option<darling::Result<Self>> {
        match meta {
            NestedMeta::Meta(syn::Meta::Path(path)) => {
                P::match_path(path).map(|p| Ok(MatchMetaPath(p?)))
            }
            _ => None,
        }
    }
}
