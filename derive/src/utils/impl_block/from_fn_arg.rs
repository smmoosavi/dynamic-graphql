use darling::util::Ignored;

pub trait FromFnArg: Sized {
    fn from_fn_arg(arg: &mut syn::FnArg) -> darling::Result<Self>;
}

#[derive(Debug, Clone)]
pub struct SelfArg {
    #[allow(dead_code)]
    pub is_mut: bool,
    #[allow(dead_code)]
    pub is_ref: bool,
    pub span: proc_macro2::Span,
}

#[derive(Debug, Clone)]
pub struct TypedArg {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

impl FromFnArg for Ignored {
    fn from_fn_arg(_arg: &mut syn::FnArg) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
