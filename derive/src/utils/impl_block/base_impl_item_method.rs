use crate::utils::impl_block::{Args, BaseFnArg, FromFnArg, FromImplItemMethod};
use crate::utils::with_index::SetIndex;

#[derive(Debug, Clone)]
pub struct BaseMethod<MethodArg = BaseFnArg> {
    pub vis: syn::Visibility,
    pub constness: bool,
    pub asyncness: bool,
    pub ident: syn::Ident,
    pub args: Args<MethodArg>,
    pub output_type: Option<syn::Type>,
}

impl<MethodArg: FromFnArg + SetIndex> FromImplItemMethod for BaseMethod<MethodArg> {
    fn from_impl_item_method(method: &mut syn::ImplItemMethod) -> darling::Result<Self> {
        Ok(BaseMethod {
            vis: method.vis.clone(),
            constness: method.sig.constness.is_some(),
            asyncness: method.sig.asyncness.is_some(),
            ident: method.sig.ident.clone(),
            args: Args::from_fn_args(&mut method.sig.inputs.iter_mut())?,
            output_type: match &method.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty.as_ref().clone()),
            },
        })
    }
}

impl<A> SetIndex for BaseMethod<A> {
    fn with_index(self, _index: usize) -> Self {
        self
    }
}
