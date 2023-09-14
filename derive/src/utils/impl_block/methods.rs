use std::ops::Deref;

use crate::utils::impl_block::FromImplItemFn;
use crate::utils::impl_block::FromTraitItemFn;
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

impl<Method: FromImplItemFn + SetIndex> Methods<Method> {
    pub fn from_impl_item_methods<'a>(
        items: &mut impl Iterator<Item = &'a mut syn::ImplItem>,
    ) -> darling::Result<Self> {
        Ok(Self {
            methods: items
                .enumerate()
                .filter_map(|(index, item)| match item {
                    syn::ImplItem::Fn(method) => {
                        Some(Method::from_impl_item_method(method).with_index(index))
                    }
                    _ => None,
                })
                .collect::<darling::Result<Vec<_>>>()?,
        })
    }
}

impl<Method: FromTraitItemFn + SetIndex> Methods<Method> {
    pub fn from_trait_item_methods<'a>(
        items: &mut impl Iterator<Item = &'a mut syn::TraitItem>,
    ) -> darling::Result<Self> {
        Ok(Self {
            methods: items
                .enumerate()
                .filter_map(|(index, item)| match item {
                    syn::TraitItem::Fn(method) => {
                        Some(Method::from_trait_item_method(method).with_index(index))
                    }
                    _ => None,
                })
                .collect::<darling::Result<Vec<_>>>()?,
        })
    }
}
