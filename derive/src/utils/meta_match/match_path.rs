pub trait MatchPath: Sized {
    fn match_path(path: &syn::Path) -> Option<darling::Result<Self>>;
}

impl MatchPath for syn::Path {
    fn match_path(path: &syn::Path) -> Option<darling::Result<Self>> {
        Some(Ok(path.clone()))
    }
}
