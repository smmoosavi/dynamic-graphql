use std::ops::{Deref, DerefMut};

use darling::ast::Data;
use darling::util::Ignored;
use darling::{FromDeriveInput, FromField, FromVariant};

use crate::utils::impl_block::{FromFnArg, FromImplItemMethod, FromItemTrait, FromTraitItemMethod};
use crate::utils::with_index::SetIndex;
use crate::FromItemImpl;

pub trait MakeContext<C: Clone> {
    fn make_context(&self) -> C;
}

impl<T> MakeContext<()> for T {
    fn make_context(&self) {}
}

impl<T> MakeContext<Ignored> for T {
    fn make_context(&self) -> Ignored {
        Ignored
    }
}

pub trait SetContext {
    type Context: Clone;
    fn set_context(&mut self, context: Self::Context);
}

impl SetContext for () {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}

impl SetContext for Ignored {
    type Context = Ignored;

    fn set_context(&mut self, _: Self::Context) {}
}

impl<T: SetContext, E> SetContext for Result<T, E> {
    type Context = T::Context;

    fn set_context(&mut self, context: Self::Context) {
        if let Ok(ref mut inner) = self {
            inner.set_context(context);
        }
    }
}

impl<T: SetContext> SetContext for Option<T> {
    type Context = T::Context;

    fn set_context(&mut self, context: Self::Context) {
        if let Some(ref mut inner) = self {
            inner.set_context(context);
        }
    }
}

impl<T: SetContext> SetContext for Vec<T> {
    type Context = T::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.iter_mut()
            .for_each(|item| item.set_context(context.clone()));
    }
}

impl<T: SetContext> SetContext for darling::ast::Fields<T> {
    type Context = T::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.fields.set_context(context);
    }
}

impl<C: Clone, V: SetContext<Context = C>, F: SetContext<Context = C>> SetContext
    for darling::ast::Data<V, F>
{
    type Context = C;

    fn set_context(&mut self, context: Self::Context) {
        match self {
            Data::Enum(vs) => {
                vs.set_context(context);
            }
            Data::Struct(st) => {
                st.fields.set_context(context);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WithContext<C, T> {
    pub ctx: C,
    pub inner: T,
}

impl<C, T> Deref for WithContext<C, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C, T> DerefMut for WithContext<C, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<C: Clone, T> SetContext for WithContext<C, T> {
    type Context = C;

    fn set_context(&mut self, context: Self::Context) {
        self.ctx = context;
    }
}

impl<C, T> SetIndex for WithContext<C, T>
where
    T: SetIndex,
{
    fn with_index(self, index: usize) -> Self {
        Self {
            ctx: self.ctx,
            inner: T::with_index(self.inner, index),
        }
    }
}

macro_rules! impl_for_with_context {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<A: $trayt, C: Default> $trayt for WithContext<C, A> {
            fn $method(input: &$syn) -> darling::Result<Self> {
                let inner = A::$method(input)?;
                Ok(WithContext {
                    ctx: C::default(),
                    inner,
                })
            }
        }
    };
}

macro_rules! impl_mut_for_with_context {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<A: $trayt, C: Default> $trayt for WithContext<C, A> {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                let inner = A::$method(input)?;
                Ok(WithContext {
                    ctx: C::default(),
                    inner,
                })
            }
        }
    };
}

impl_for_with_context!(FromDeriveInput, from_derive_input, syn::DeriveInput);
impl_for_with_context!(FromField, from_field, syn::Field);
impl_for_with_context!(FromVariant, from_variant, syn::Variant);

impl_mut_for_with_context!(FromFnArg, from_fn_arg, syn::FnArg);
impl_mut_for_with_context!(
    FromImplItemMethod,
    from_impl_item_method,
    syn::ImplItemMethod
);
impl_mut_for_with_context!(FromItemImpl, from_item_impl, syn::ItemImpl);
impl_mut_for_with_context!(FromItemTrait, from_item_trait, syn::ItemTrait);
impl_mut_for_with_context!(
    FromTraitItemMethod,
    from_trait_item_method,
    syn::TraitItemMethod
);
