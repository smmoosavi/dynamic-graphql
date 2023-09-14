use std::ops::Deref;
use std::ops::DerefMut;

use syn::FnArg;
use syn::ImplItemFn;
use syn::TraitItemFn;

use crate::utils::impl_block::FromFnArg;
use crate::utils::impl_block::FromImplItemFn;
use crate::utils::impl_block::FromTraitItemFn;
use crate::utils::with_context::SetContext;

pub trait SetIndex: Sized {
    fn with_index(self, index: usize) -> Self;
}

impl<T: SetIndex, E> SetIndex for Result<T, E> {
    fn with_index(self, index: usize) -> Self {
        self.map(|t| T::with_index(t, index))
    }
}

#[derive(Debug, Clone)]
pub struct WithIndex<D> {
    pub index: usize,
    pub inner: D,
}

impl<D> Deref for WithIndex<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D> DerefMut for WithIndex<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<D> AsRef<D> for WithIndex<D> {
    fn as_ref(&self) -> &D {
        &self.inner
    }
}

impl<A> SetIndex for WithIndex<A> {
    fn with_index(self, index: usize) -> Self {
        WithIndex {
            index,
            inner: self.inner,
        }
    }
}

impl<A: FromFnArg> FromFnArg for WithIndex<A> {
    fn from_fn_arg(arg: &mut FnArg) -> darling::Result<Self> {
        let inner = A::from_fn_arg(arg)?;
        Ok(WithIndex {
            index: usize::MAX,
            inner,
        })
    }
}

impl<A: FromImplItemFn> FromImplItemFn for WithIndex<A> {
    fn from_impl_item_method(method: &mut ImplItemFn) -> darling::Result<Self> {
        let inner = A::from_impl_item_method(method)?;
        Ok(WithIndex {
            index: usize::MAX,
            inner,
        })
    }
}

impl<A: FromTraitItemFn> FromTraitItemFn for WithIndex<A> {
    fn from_trait_item_method(method: &mut TraitItemFn) -> darling::Result<Self>
    where
        Self: Sized,
    {
        let inner = A::from_trait_item_method(method)?;
        Ok(WithIndex {
            index: usize::MAX,
            inner,
        })
    }
}

impl<T: SetContext> SetContext for WithIndex<T> {
    type Context = T::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.inner.set_context(context);
    }
}
