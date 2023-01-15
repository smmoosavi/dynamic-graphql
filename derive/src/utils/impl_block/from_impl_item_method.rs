use darling::util::Ignored;

pub trait FromImplItemMethod: Sized {
    fn from_impl_item_method(impl_item_method: &mut syn::ImplItemMethod) -> darling::Result<Self>;
}

impl FromImplItemMethod for Ignored {
    fn from_impl_item_method(_method: &mut syn::ImplItemMethod) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
