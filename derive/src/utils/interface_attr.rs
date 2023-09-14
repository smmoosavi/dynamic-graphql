use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::utils::meta_match::MatchMetaPath;
use crate::utils::meta_match::MatchNestedMetaList;

#[derive(Debug, Clone)]
pub struct InterfaceMarkAttr {
    pub path: syn::Path,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct InterfaceImplAttr {
    pub path: syn::Path,
    pub span: Span,
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
        let mark = MatchMarkWith::match_nested_meta_list(items);

        if let Some(r) = mark {
            return r.map(|mark| {
                let span = mark.0.span();
                InterfaceMarkAttr {
                    path: mark.0 .0,
                    span,
                }
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
