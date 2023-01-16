use crate::utils::impl_block::{
    FromFnArg, FromImplItemMethod, FromItemImpl, FromItemTrait, FromTraitItemMethod,
};
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use darling::util::Ignored;
use darling::{FromDeriveInput, FromField, FromVariant};
use std::ops::Deref;
use syn::DeriveInput;

pub trait SetArg<Arg> {
    type Output: Sized;
    fn with_arg(self, arg: Arg) -> Self::Output;
}

impl<T, E, A> SetArg<A> for Result<T, E>
where
    T: SetArg<A>,
{
    type Output = Result<T::Output, E>;

    fn with_arg(self, arg: A) -> Self::Output {
        self.map(|t| t.with_arg(arg))
    }
}

pub struct WithArg<Arg, D> {
    pub arg: Arg,
    pub inner: D,
}

impl<D, Arg> Deref for WithArg<Arg, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D, Arg> SetArg<Arg> for WithArg<Ignored, D> {
    type Output = WithArg<Arg, D>;

    fn with_arg(self, arg: Arg) -> Self::Output {
        WithArg {
            arg,
            inner: self.inner,
        }
    }
}

macro_rules! impl_set_arg {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<D: $trayt> $trayt for WithArg<Ignored, D> {
            fn $method(input: &$syn) -> darling::Result<Self> {
                let inner = D::$method(input)?;
                Ok(WithArg {
                    arg: Ignored,
                    inner,
                })
            }
        }
    };
}

impl_set_arg!(FromDeriveInput, from_derive_input, DeriveInput);
impl_set_arg!(FromField, from_field, syn::Field);
impl_set_arg!(FromVariant, from_variant, syn::Variant);

macro_rules! impl_mut_set_arg {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<D: $trayt> $trayt for WithArg<Ignored, D> {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                let inner = D::$method(input)?;
                Ok(WithArg {
                    arg: Ignored,
                    inner,
                })
            }
        }
    };
}

impl_mut_set_arg!(FromFnArg, from_fn_arg, syn::FnArg);
impl_mut_set_arg!(
    FromImplItemMethod,
    from_impl_item_method,
    syn::ImplItemMethod
);
impl_mut_set_arg!(FromItemImpl, from_item_impl, syn::ItemImpl);
impl_mut_set_arg!(FromItemTrait, from_item_trait, syn::ItemTrait);
impl_mut_set_arg!(
    FromTraitItemMethod,
    from_trait_item_method,
    syn::TraitItemMethod
);

impl<A, D: SetIndex> SetIndex for WithArg<A, D> {
    fn with_index(self, index: usize) -> Self {
        WithArg {
            arg: self.arg,
            inner: self.inner.with_index(index),
        }
    }
}

impl<A, D: SetContext> SetContext for WithArg<A, D> {
    type Context = D::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.inner.set_context(context);
    }
}
