use darling::util::Ignored;

pub trait FromTraitItemFn {
    fn from_trait_item_method(trait_item_method: &mut syn::TraitItemFn) -> darling::Result<Self>
    where
        Self: Sized;
}
impl FromTraitItemFn for Ignored {
    fn from_trait_item_method(_method: &mut syn::TraitItemFn) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
