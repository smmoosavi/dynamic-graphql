use crate::utils::impl_block::Args;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::impl_block::FromFnArg;
use crate::utils::impl_block::FromImplItemFn;
use crate::utils::impl_block::FromSignature;
use crate::utils::impl_block::FromTraitItemFn;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;

#[derive(Debug, Clone)]
pub struct BaseMethod<MethodArg = BaseFnArg> {
    pub asyncness: bool,
    pub ident: syn::Ident,
    pub args: Args<MethodArg>,
    pub output_type: Option<syn::Type>,
}

impl<MethodArg: FromFnArg + SetIndex> FromSignature for BaseMethod<MethodArg> {
    fn from_signature(sig: &mut syn::Signature) -> darling::Result<Self> {
        Ok(BaseMethod {
            asyncness: sig.asyncness.is_some(),
            ident: sig.ident.clone(),
            args: Args::from_fn_args(&mut sig.inputs.iter_mut())?,
            output_type: match &sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty.as_ref().clone()),
            },
        })
    }
}

impl<MethodArg: FromFnArg + SetIndex> FromImplItemFn for BaseMethod<MethodArg> {
    fn from_impl_item_method(impl_item_method: &mut syn::ImplItemFn) -> darling::Result<Self> {
        Self::from_signature(&mut impl_item_method.sig)
    }
}

impl<MethodArg: FromFnArg + SetIndex> FromTraitItemFn for BaseMethod<MethodArg> {
    fn from_trait_item_method(trait_item_method: &mut syn::TraitItemFn) -> darling::Result<Self> {
        Self::from_signature(&mut trait_item_method.sig)
    }
}

impl<A> SetIndex for BaseMethod<A> {
    fn with_index(self, _index: usize) -> Self {
        self
    }
}

impl<MethodArg> SetContext for BaseMethod<MethodArg> {
    type Context = ();

    fn set_context(&mut self, _context: Self::Context) {}
}
