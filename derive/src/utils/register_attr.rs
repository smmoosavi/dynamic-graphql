use darling::FromMeta;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::NestedMeta;

use crate::utils::meta_match::MatchMetaPath;
use crate::utils::meta_match::MatchNestedMetaList;

#[derive(Debug, Clone)]
pub struct RegisterAttr {
    pub path: syn::Path,
    pub span: Span,
}

struct MatchRegister(MatchMetaPath);

impl MatchNestedMetaList for MatchRegister {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>>
    where
        Self: Sized,
    {
        let inner = <(MatchMetaPath,)>::match_nested_meta_list(list);
        inner.map(|r| r.map(|(r1,)| MatchRegister(r1)))
    }
}

impl FromMeta for RegisterAttr {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let implements = MatchRegister::match_nested_meta_list(items);
        if let Some(r) = implements {
            return r.map(|r| {
                let span = r.0.span();
                RegisterAttr { path: r.0 .0, span }
            });
        }

        Err(darling::Error::custom("Invalid register attribute"))
    }
}

impl ToTokens for RegisterAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.path;
        tokens.extend(quote! {
            let registry = registry.register::<#path>();
        });
    }
}

impl RegisterAttr {
    pub fn with_generic(&mut self, ty: syn::Type) {
        if let Some(segment) = self.path.segments.last_mut() {
            let mut args = syn::punctuated::Punctuated::new();
            args.push(syn::GenericArgument::Type(ty));
            segment.arguments =
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: syn::token::Lt::default(),
                    args,
                    gt_token: syn::token::Gt::default(),
                });
        }
    }
}
