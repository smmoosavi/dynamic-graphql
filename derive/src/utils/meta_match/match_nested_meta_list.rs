pub trait MatchNestedMetaList {
    fn match_nested_meta_list(list: &[syn::NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized;
}
