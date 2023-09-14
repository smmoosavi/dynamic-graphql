use darling::ast::NestedMeta;
use darling::FromMeta;

use crate::utils::meta_match::MatchMetaPath;
use crate::utils::meta_match::MatchNestedMetaList;

#[derive(Debug, Clone)]
pub struct PathAttr(pub syn::Path);

impl MatchNestedMetaList for PathAttr {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchMetaPath,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| PathAttr(r1.0)))
    }
}

impl FromMeta for PathAttr {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let path = PathAttr::match_nested_meta_list(items);
        if let Some(p) = path {
            return p;
        }
        Err(darling::Error::custom("Invalid path"))
    }
}
