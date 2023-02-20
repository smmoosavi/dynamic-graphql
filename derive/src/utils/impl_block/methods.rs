use std::ops::Deref;

use crate::utils::impl_block::FromImplItemMethod;
use crate::utils::impl_block::FromTraitItemMethod;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;

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

impl<Method: FromImplItemMethod + SetIndex> Methods<Method> {
    pub fn from_impl_item_methods<'a>(
        items: &mut impl Iterator<Item = &'a mut syn::ImplItem>,
    ) -> darling::Result<Self> {
        Ok(Self {
            methods: items
                .enumerate()
                .filter_map(|(index, item)| match item {
                    syn::ImplItem::Method(method) => {
                        Some(Method::from_impl_item_method(method).with_index(index))
                    }
                    _ => None,
                })
                .collect::<darling::Result<Vec<_>>>()?,
        })
    }
}

impl<Method: FromTraitItemMethod + SetIndex> Methods<Method> {
    pub fn from_trait_item_methods<'a>(
        items: &mut impl Iterator<Item = &'a mut syn::TraitItem>,
    ) -> darling::Result<Self> {
        Ok(Self {
            methods: items
                .enumerate()
                .filter_map(|(index, item)| match item {
                    syn::TraitItem::Method(method) => {
                        Some(Method::from_trait_item_method(method).with_index(index))
                    }
                    _ => None,
                })
                .collect::<darling::Result<Vec<_>>>()?,
        })
    }
}
