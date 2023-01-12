use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::method::BaseMethod;
use crate::utils::impl_block::method::FromMethod;
use crate::utils::with_context::SetContext;
use darling::util::Ignored;
use std::ops::Deref;

pub trait FromItemImpl: Sized {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> GeneratorResult<Self>;
}

#[derive(Debug, Clone)]
pub struct BaseItemImpl<Method = BaseMethod> {
    pub trait_: Option<syn::Path>,
    pub ty: syn::Type,
    pub methods: Methods<Method>,
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

impl<Method: FromMethod> FromItemImpl for BaseItemImpl<Method> {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> GeneratorResult<Self> {
        Ok(Self {
            trait_: item_impl.trait_.as_ref().map(|t| t.1.clone()),
            ty: item_impl.self_ty.as_ref().clone(),
            methods: Methods {
                methods: item_impl
                    .items
                    .iter_mut()
                    .filter_map(|item| match item {
                        syn::ImplItem::Method(method) => Some(Method::from_method(method)),
                        _ => None,
                    })
                    .collect::<GeneratorResult<Vec<_>>>()?,
            },
        })
    }
}

impl FromItemImpl for Ignored {
    fn from_item_impl(_item_impl: &mut syn::ItemImpl) -> GeneratorResult<Self> {
        Ok(Ignored)
    }
}
