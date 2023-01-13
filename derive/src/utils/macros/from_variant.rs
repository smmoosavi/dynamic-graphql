macro_rules! from_variant {
    ($name:ident, $ty:ty,) => {
        from_variant!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from!(darling::FromVariant, from_variant, syn::Variant, $name,);
    };
}

pub(crate) use from_variant;
