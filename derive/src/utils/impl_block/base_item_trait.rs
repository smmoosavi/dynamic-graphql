use darling::FromGenerics;

use crate::utils::impl_block::from_item_trait::FromItemTrait;
use crate::utils::impl_block::from_trait_item_method::FromTraitItemFn;
use crate::utils::impl_block::BaseMethod;
use crate::utils::impl_block::Methods;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;

#[derive(Clone, Debug)]
pub struct BaseItemTrait<Method = BaseMethod, Generics = ()> {
    pub ident: syn::Ident,
    pub generics: Generics,
    pub methods: Methods<Method>,
    // todo consts, types, super_traits
}

impl<Method, Generics> FromItemTrait for BaseItemTrait<Method, Generics>
where
    Method: FromTraitItemFn + SetIndex,
    Generics: FromGenerics,
{
    fn from_item_trait(item_trait: &mut syn::ItemTrait) -> darling::Result<Self> {
        Ok(Self {
            ident: item_trait.ident.clone(),
            generics: FromGenerics::from_generics(&item_trait.generics)?,
            methods: Methods::from_trait_item_methods(&mut item_trait.items.iter_mut())?,
        })
    }
}

impl<Method, Generics> SetContext for BaseItemTrait<Method, Generics>
where
    Method: FromTraitItemFn + SetContext,
{
    type Context = Method::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.methods.set_context(context);
    }
}
