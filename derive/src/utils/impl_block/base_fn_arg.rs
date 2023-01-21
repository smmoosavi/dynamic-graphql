use crate::utils::impl_block::{FromFnArg, SelfArg, TypedArg};
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use darling::util::Ignored;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub enum BaseFnArg {
    Receiver(SelfArg),
    Typed(TypedArg),
}

impl BaseFnArg {
    #[allow(dead_code)]
    pub fn get_attrs(arg: &syn::FnArg) -> &Vec<syn::Attribute> {
        match arg {
            syn::FnArg::Receiver(r) => &r.attrs,
            syn::FnArg::Typed(t) => &t.attrs,
        }
    }
    pub fn get_attrs_mut(arg: &mut syn::FnArg) -> &mut Vec<syn::Attribute> {
        match arg {
            syn::FnArg::Receiver(r) => &mut r.attrs,
            syn::FnArg::Typed(t) => &mut t.attrs,
        }
    }
}

impl Spanned for BaseFnArg {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Receiver(r) => r.span,
            Self::Typed(t) => t.ident.span(),
        }
    }
}

impl SetIndex for BaseFnArg {
    fn with_index(self, _index: usize) -> Self {
        self
    }
}

impl SetContext for BaseFnArg {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}

impl FromFnArg for BaseFnArg {
    fn from_fn_arg(arg: &mut syn::FnArg) -> darling::Result<Self> {
        match arg {
            syn::FnArg::Receiver(receiver) => Ok(Self::Receiver(SelfArg {
                is_mut: receiver.mutability.is_some(),
                is_ref: receiver.reference.is_some(),
                span: receiver.self_token.span(),
            })),
            syn::FnArg::Typed(typed) => Ok({
                let ident = match *typed.pat {
                    syn::Pat::Ident(ref i) => i.ident.clone(),
                    _ => {
                        return Err(darling::Error::unsupported_shape("unnamed arguments")
                            .with_span(&typed.pat));
                    }
                };
                Self::Typed(TypedArg {
                    ident,
                    ty: typed.ty.as_ref().clone(),
                })
            }),
        }
    }
}
