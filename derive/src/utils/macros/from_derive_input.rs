macro_rules! from_derive_input {
    ($name:ident, $ty:ty,) => {
        from_derive_input!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from!(
            darling::FromDeriveInput,
            from_derive_input,
            syn::DeriveInput,
            $name,
        );
    };
    ($name:ident, $ty:ty, ctx,) => {
        from_derive_input!($name, $ty, ctx);
    };
    ($name:ident, $ty:ty, ctx) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from!(
            darling::FromDeriveInput,
            from_derive_input,
            syn::DeriveInput,
            $name,
            ctx,
        );
    };
    ($name:ident, $ty:ty, inner=$( $field_path:ident ).+,) => {
        from_derive_input!($name, $ty, inner=$( $field_path ).+);
    };
    ($name:ident, $ty:ty, inner=$( $field_path:ident ).+) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty, inner=$( $field_path ).+);

        impl_from!(
            darling::FromDeriveInput,
            from_derive_input,
            syn::DeriveInput,
            $name,
            inner=$( $field_path ).+,
        );

    };
}

pub(crate) use from_derive_input;
