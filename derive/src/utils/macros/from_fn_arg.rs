macro_rules! from_fn_arg {
    ($name:ident, $ty:ty,) => {
        from_fn_arg!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);
        crate::utils::macros::set_index!($name);

        impl_from_mut!(
            crate::utils::impl_block::FromFnArg,
            from_fn_arg,
            syn::FnArg,
            $name,
        );
    };
}

pub(crate) use from_fn_arg;
