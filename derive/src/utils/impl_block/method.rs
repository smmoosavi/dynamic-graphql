use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::fn_arg::BaseFnArg;
use crate::utils::impl_block::fn_arg::FromFnArg;
use darling::util::Ignored;
use std::ops::Deref;

pub trait FromMethod: Sized {
    fn from_method(method: &mut syn::ImplItemMethod) -> GeneratorResult<Self>;
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

impl<MethodArg: FromFnArg> FromMethod for BaseMethod<MethodArg> {
    fn from_method(method: &mut syn::ImplItemMethod) -> GeneratorResult<Self> {
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
                    .map(|arg| MethodArg::from_fn_arg(arg))
                    .collect::<GeneratorResult<Vec<_>>>()?,
            },
            output_type: match &method.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty.as_ref().clone()),
            },
        })
    }
}

impl FromMethod for Ignored {
    fn from_method(_method: &mut syn::ImplItemMethod) -> GeneratorResult<Self> {
        Ok(Ignored)
    }
}
