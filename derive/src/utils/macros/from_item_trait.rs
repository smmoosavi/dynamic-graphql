macro_rules! from_item_trait {
    ($name:ident, $ty:ty,) => {
        from_item_trait!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from_mut!(
            crate::utils::impl_block::FromItemTrait,
            from_item_trait,
            syn::ItemTrait,
            $name,
        );
    };
    ($name:ident, $ty:ty, ctx,) => {
        from_item_trait!($name, $ty, ctx);
    };
    ($name:ident, $ty:ty, ctx) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from_mut!(
            crate::utils::impl_block::FromItemTrait,
            from_item_trait,
            syn::ItemTrait,
            $name,
            ctx,
        );
    };
}

pub(crate) use from_item_trait;
