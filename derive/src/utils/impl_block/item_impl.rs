use crate::utils::impl_block::impl_item_method::BaseMethod;
use crate::utils::impl_block::impl_item_method::FromImplItemMethod;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use darling::util::Ignored;
use darling::FromGenerics;
use std::ops::Deref;

pub trait FromItemImpl: Sized {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> darling::Result<Self>;
}

#[derive(Debug, Clone)]
pub struct BaseItemImpl<Method = BaseMethod, Generics = ()> {
    pub trait_: Option<syn::Path>,
    pub ty: syn::Type,
    pub methods: Methods<Method>,
    pub generics: Generics,
    // todo generics, consts, types
}

#[derive(Debug, Clone)]
pub struct Methods<Method> {
    pub methods: Vec<Method>,
}

impl<Method> Deref for Methods<Method> {
    type Target = Vec<Method>;

    fn deref(&self) -> &Self::Target {
        &self.methods
    }
}

impl<Method: SetContext> SetContext for Methods<Method> {
    type Context = Method::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.methods.set_context(context);
    }
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

impl FromItemImpl for Ignored {
    fn from_item_impl(_item_impl: &mut syn::ItemImpl) -> darling::Result<Self> {
        Ok(Ignored)
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
