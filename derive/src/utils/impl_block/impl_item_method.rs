use crate::utils::impl_block::{BaseFnArg, FromFnArg};
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use darling::util::Ignored;
use std::ops::Deref;

pub trait FromImplItemMethod: Sized {
    fn from_impl_item_method(impl_item_method: &mut syn::ImplItemMethod) -> darling::Result<Self>;
}

#[derive(Debug, Clone)]
pub struct BaseMethod<MethodArg = BaseFnArg> {
    pub vis: syn::Visibility,
    pub constness: bool,
    pub asyncness: bool,
    pub ident: syn::Ident,
    pub args: Args<MethodArg>,
    pub output_type: Option<syn::Type>,
}

#[derive(Debug, Clone)]
pub struct Args<MethodArg> {
    pub args: Vec<MethodArg>,
}

impl<MethodArg> Deref for Args<MethodArg> {
    type Target = Vec<MethodArg>;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl<MethodArg: SetContext> SetContext for Args<MethodArg> {
    type Context = MethodArg::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.args.set_context(context);
    }
}

impl<MethodArg: FromFnArg + SetIndex> FromImplItemMethod for BaseMethod<MethodArg> {
    fn from_impl_item_method(method: &mut syn::ImplItemMethod) -> darling::Result<Self> {
        Ok(BaseMethod {
            vis: method.vis.clone(),
            constness: method.sig.constness.is_some(),
            asyncness: method.sig.asyncness.is_some(),
            ident: method.sig.ident.clone(),
            args: Args {
                args: method
                    .sig
                    .inputs
                    .iter_mut()
                    .enumerate()
                    .map(|(index, arg)| MethodArg::from_fn_arg(arg).with_index(index))
                    .collect::<darling::Result<Vec<_>>>()?,
            },
            output_type: match &method.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty.as_ref().clone()),
            },
        })
    }
}

impl FromImplItemMethod for Ignored {
    fn from_impl_item_method(_method: &mut syn::ImplItemMethod) -> darling::Result<Self> {
        Ok(Ignored)
    }
}

impl<A> SetIndex for BaseMethod<A> {
    fn with_index(self, _index: usize) -> Self {
        self
    }
}
