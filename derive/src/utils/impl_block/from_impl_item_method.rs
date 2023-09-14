use darling::util::Ignored;

pub trait FromImplItemFn: Sized {
    fn from_impl_item_method(impl_item_method: &mut syn::ImplItemFn) -> darling::Result<Self>;
}

impl FromImplItemFn for Ignored {
    fn from_impl_item_method(_method: &mut syn::ImplItemFn) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
