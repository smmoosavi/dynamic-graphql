use darling::ast::NestedMeta;

pub trait MatchNestedMeta: Sized {
    fn match_nested_meta(meta: &NestedMeta) -> Option<darling::Result<Self>>;
}
