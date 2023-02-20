use darling::FromGenerics;

use crate::utils::impl_block::BaseMethod;
use crate::utils::impl_block::FromImplItemMethod;
use crate::utils::impl_block::Methods;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use crate::FromItemImpl;

#[derive(Debug, Clone)]
pub struct BaseItemImpl<Method = BaseMethod, Generics = ()> {
    pub trait_: Option<syn::Path>,
    pub ty: syn::Type,
    pub methods: Methods<Method>,
    pub generics: Generics,
    // todo generics, consts, types
}

impl<Method, Generics> FromItemImpl for BaseItemImpl<Method, Generics>
where
    Method: FromImplItemMethod + SetIndex,
    Generics: FromGenerics,
{
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> darling::Result<Self> {
        Ok(Self {
            trait_: item_impl.trait_.as_ref().map(|t| t.1.clone()),
            ty: item_impl.self_ty.as_ref().clone(),
            generics: FromGenerics::from_generics(&item_impl.generics)?,
            methods: Methods::from_impl_item_methods(&mut item_impl.items.iter_mut())?,
        })
    }
}

impl<Method, Generics> SetContext for BaseItemImpl<Method, Generics>
where
    Method: FromImplItemMethod + SetContext,
{
    type Context = Method::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.methods.set_context(context);
    }
}
