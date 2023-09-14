macro_rules! from_impl_item_method {
    ($name:ident, $ty:ty,) => {
        from_impl_item_method!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);
        crate::utils::macros::set_index!($name);

        crate::utils::macros::impl_from_mut!(
            crate::utils::impl_block::FromImplItemFn,
            from_impl_item_method,
            syn::ImplItemFn,
            $name,
        );

    };
    ($name:ident, $ty:ty, ctx,) => {
        from_impl_item_method!($name, $ty, ctx);
    };
    ($name:ident, $ty:ty, ctx) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);
        crate::utils::macros::set_index!($name);


        crate::utils::macros::impl_from_mut!(
            crate::utils::impl_block::FromImplItemFn,
            from_impl_item_method,
            syn::ImplItemFn,
            $name,
            ctx,
        );
    };

    ($name:ident, $ty:ty, inner=$( $field_path:ident ).+,) => {
        from_impl_item_method!($name, $ty, inner=$( $field_path ).+);
    };
    ($name:ident, $ty:ty, inner=$( $field_path:ident ).+) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty, inner=$( $field_path ).+);
        crate::utils::macros::set_index!($name);


        crate::utils::macros::impl_from_mut!(
            crate::utils::impl_block::FromImplItemFn,
            from_impl_item_method,
            syn::ImplItemFn,
            $name,
            inner=$( $field_path ).+,
        );

    };

}

pub(crate) use from_impl_item_method;
