macro_rules! from_fn_arg {
    ($name:ident, $ty:ty,) => {
        from_fn_arg!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from_mut!(
            crate::utils::impl_block::FromFnArg,
            from_fn_arg,
            syn::FnArg,
            $name,
        );

        impl crate::utils::with_index::SetIndex for $name {
            fn with_index(self, index: usize) -> Self {
                Self(crate::utils::with_index::SetIndex::with_index(
                    self.0, index,
                ))
            }
        }
    };
}

pub(crate) use from_fn_arg;
