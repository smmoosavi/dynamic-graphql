use darling::FromMeta;
use syn::NestedMeta;

use crate::utils::meta_match::{MatchMetaPath, MatchNestedMetaList};

#[derive(Debug, Clone)]
pub struct RemoteAttr(pub syn::Path);

impl MatchNestedMetaList for RemoteAttr {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchMetaPath,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| RemoteAttr(r1.0)))
    }
}

impl FromMeta for RemoteAttr {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let remote = RemoteAttr::match_nested_meta_list(items);
        if let Some(r) = remote {
            return r;
        }
        Err(darling::Error::custom("Invalid remote attribute"))
    }
}
