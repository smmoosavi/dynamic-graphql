use darling::util::Ignored;

pub trait FromItemTrait {
    fn from_item_trait(item_trait: &mut syn::ItemTrait) -> darling::Result<Self>
    where
        Self: Sized;
}
impl FromItemTrait for Ignored {
    fn from_item_trait(_item_trait: &mut syn::ItemTrait) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
