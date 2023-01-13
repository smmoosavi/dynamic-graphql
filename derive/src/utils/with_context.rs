use crate::utils::impl_block::{FromFnArg, FromMethod};
use crate::FromItemImpl;
use darling::ast::Data;
use darling::util::Ignored;
use darling::{FromDeriveInput, FromField, FromVariant};
use std::ops::{Deref, DerefMut};
use syn::{DeriveInput, Field, ImplItemMethod, Variant};

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

impl<A: FromFnArg, C: Default> FromFnArg for WithContext<C, A> {
    fn from_fn_arg(arg: &mut syn::FnArg) -> darling::Result<Self> {
        let inner = A::from_fn_arg(arg)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}

impl<A: FromMethod, C: Default> FromMethod for WithContext<C, A> {
    fn from_method(method: &mut ImplItemMethod) -> darling::Result<Self> {
        let inner = A::from_method(method)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}

impl<A: FromItemImpl, C: Default> FromItemImpl for WithContext<C, A> {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> darling::Result<Self> {
        let inner = A::from_item_impl(item_impl)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}

impl<A: FromDeriveInput, C: Default> FromDeriveInput for WithContext<C, A> {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let inner = A::from_derive_input(input)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}

impl<A: FromField, C: Default> FromField for WithContext<C, A> {
    fn from_field(field: &Field) -> darling::Result<Self> {
        let inner = A::from_field(field)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}

impl<A: FromVariant, C: Default> FromVariant for WithContext<C, A> {
    fn from_variant(variant: &Variant) -> darling::Result<Self> {
        let inner = A::from_variant(variant)?;
        Ok(WithContext {
            ctx: C::default(),
            inner,
        })
    }
}
