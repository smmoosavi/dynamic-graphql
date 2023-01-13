macro_rules! from_item_impl {
    ($name:ident, $ty:ty,) => {
        from_item_impl!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from_mut!(
            crate::utils::impl_block::FromItemImpl,
            from_item_impl,
            syn::ItemImpl,
            $name,
        );
    };
    ($name:ident, $ty:ty, ctx,) => {
        from_item_impl!($name, $ty, ctx);
    };
    ($name:ident, $ty:ty, ctx) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from_mut!(
            crate::utils::impl_block::FromItemImpl,
            from_item_impl,
            syn::ItemImpl,
            $name,
            ctx,
        );
    };
}

pub(crate) use from_item_impl;
