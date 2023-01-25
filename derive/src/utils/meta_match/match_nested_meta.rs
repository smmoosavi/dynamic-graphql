pub trait MatchNestedMeta: Sized {
    fn match_nested_meta(meta: &syn::NestedMeta) -> Option<darling::Result<Self>>;
}
