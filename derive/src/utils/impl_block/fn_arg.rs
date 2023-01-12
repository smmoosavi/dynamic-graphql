use crate::utils::error::GeneratorResult;
use crate::utils::with_index::SetIndex;
use darling::util::Ignored;
use syn::spanned::Spanned;

pub trait FromFnArg: Sized {
    fn from_fn_arg(arg: &mut syn::FnArg) -> GeneratorResult<Self>;
}

#[derive(Debug, Clone)]
pub struct SelfArg {
    pub is_mut: bool,
    pub is_ref: bool,
    pub span: proc_macro2::Span,
}

#[derive(Debug, Clone)]
pub struct TypedArg {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

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
    type Output = Self;
    fn with_index(self, _index: usize) -> Self {
        self
    }
}

impl FromFnArg for BaseFnArg {
    fn from_fn_arg(arg: &mut syn::FnArg) -> GeneratorResult<Self> {
        match arg {
            syn::FnArg::Receiver(receiver) => Ok(Self::Receiver(SelfArg {
                is_mut: receiver.mutability.is_some(),
                is_ref: receiver.reference.is_some(),
                span: receiver.span(),
            })),
            syn::FnArg::Typed(typed) => Ok({
                let ident = match *typed.pat {
                    syn::Pat::Ident(ref i) => i.ident.clone(),
                    _ => {
                        return Err(syn::Error::new(
                            typed.pat.span(),
                            "Only named arguments are supported",
                        )
                        .into());
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

impl FromFnArg for Ignored {
    fn from_fn_arg(_arg: &mut syn::FnArg) -> GeneratorResult<Self> {
        Ok(Ignored)
    }
}
