use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::FromFnArg;
use crate::utils::with_context::SetContext;
use std::ops::Deref;
use syn::FnArg;

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
    fn from_fn_arg(arg: &mut FnArg) -> GeneratorResult<Self> {
        let inner = A::from_fn_arg(arg)?;
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
