use crate::utils::meta_match::{MatchLitStr, MatchMetaPath, MatchNestedMetaList};
use darling::FromMeta;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::NestedMeta;

#[derive(Debug, Clone)]
pub enum InterfaceMarkAttr {
    MarkAs(String, Span),
    MarkWith(syn::Path, Span),
}

impl Spanned for InterfaceMarkAttr {
    fn span(&self) -> Span {
        match self {
            InterfaceMarkAttr::MarkAs(_, span) => *span,
            InterfaceMarkAttr::MarkWith(_, span) => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceImplAttr {
    pub path: syn::Path,
    pub span: Span,
}

impl Spanned for InterfaceImplAttr {
    fn span(&self) -> Span {
        self.span
    }
}

struct MatchMarkAs(MatchLitStr);

impl MatchNestedMetaList for MatchMarkAs {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchLitStr,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| MatchMarkAs(r1)))
    }
}

struct MatchMarkWith(MatchMetaPath);

impl MatchNestedMetaList for MatchMarkWith {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchMetaPath,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| MatchMarkWith(r1)))
    }
}

struct MatchImplements(MatchMetaPath);

impl MatchNestedMetaList for MatchImplements {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchMetaPath,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| MatchImplements(r1)))
    }
}

impl FromMeta for InterfaceMarkAttr {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mark_with = MatchMarkWith::match_nested_meta_list(items);

        if let Some(r) = mark_with {
            return r.map(|r| {
                let span = r.0.span();
                InterfaceMarkAttr::MarkWith(r.0 .0, span)
            });
        }

        let mark_as = MatchMarkAs::match_nested_meta_list(items);
        if let Some(r) = mark_as {
            return r.map(|r| {
                let span = r.0.span();
                InterfaceMarkAttr::MarkAs(r.0 .0, span)
            });
        }

        Err(darling::Error::custom("Invalid interface attribute"))
    }
}

impl FromMeta for InterfaceImplAttr {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let implements = MatchImplements::match_nested_meta_list(items);
        if let Some(r) = implements {
            return r.map(|r| {
                let span = r.0.span();
                InterfaceImplAttr { path: r.0 .0, span }
            });
        }

        Err(darling::Error::custom("Invalid interface attribute"))
    }
}
