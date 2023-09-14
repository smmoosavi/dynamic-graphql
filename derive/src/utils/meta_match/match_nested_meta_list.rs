use darling::ast::NestedMeta;

pub trait MatchNestedMetaList {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized;
}
