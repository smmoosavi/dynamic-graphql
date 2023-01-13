macro_rules! from_field {
    ($name:ident, $ty:ty,) => {
        from_field!($name, $ty);
    };
    ($name:ident, $ty:ty) => {
        crate::utils::macros::define!($name, $ty);
        crate::utils::macros::deref!($name, $ty);
        crate::utils::macros::set_context!($name, $ty);

        impl_from!(darling::FromField, from_field, syn::Field, $name,);
    };
}

pub(crate) use from_field;
