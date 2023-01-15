use darling::util::Ignored;

pub trait FromTraitItemMethod {
    fn from_trait_item_method(
        trait_item_method: &mut syn::TraitItemMethod,
    ) -> darling::Result<Self>
    where
        Self: Sized;
}
impl FromTraitItemMethod for Ignored {
    fn from_trait_item_method(_method: &mut syn::TraitItemMethod) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
