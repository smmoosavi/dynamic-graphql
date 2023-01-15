use darling::util::Ignored;

pub trait FromItemImpl: Sized {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> darling::Result<Self>;
}

impl FromItemImpl for Ignored {
    fn from_item_impl(_item_impl: &mut syn::ItemImpl) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
