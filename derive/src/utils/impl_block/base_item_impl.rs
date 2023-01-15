use crate::utils::impl_block::{BaseMethod, FromImplItemMethod, Methods};
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use crate::FromItemImpl;
use darling::FromGenerics;

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
            methods: Methods {
                methods: item_impl
                    .items
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(index, item)| match item {
                        syn::ImplItem::Method(method) => {
                            Some(Method::from_impl_item_method(method).with_index(index))
                        }
                        _ => None,
                    })
                    .collect::<darling::Result<Vec<_>>>()?,
            },
        })
    }
}

impl<Method, Generics> SetContext for BaseItemImpl<Method, Generics>
where
    Method: FromImplItemMethod + SetContext,
    Generics: FromGenerics,
{
    type Context = Method::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.methods.set_context(context);
    }
}
