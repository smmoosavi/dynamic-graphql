use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::FromFnArg;
use std::ops::Deref;
use syn::FnArg;

pub trait SetIndexBuilder {
    type Builder: SetIndex<Output = Self>;
}

pub trait SetIndex: Sized {
    type Output;
    fn with_index(self, index: usize) -> Self::Output;
}

impl<T: SetIndex, E> SetIndex for Result<T, E> {
    type Output = Result<T::Output, E>;
    fn with_index(self, index: usize) -> Self::Output {
        self.map(|t| T::with_index(t, index))
    }
}

#[derive(Debug, Clone)]
pub struct NeedIndex<D> {
    pub inner: D,
}

impl<D> SetIndex for NeedIndex<D> {
    type Output = WithIndex<D>;
    fn with_index(self, index: usize) -> Self::Output {
        WithIndex {
            index,
            inner: self.inner,
        }
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

impl<A> SetIndexBuilder for WithIndex<A> {
    type Builder = NeedIndex<A>;
}

impl<A> SetIndex for WithIndex<A> {
    type Output = Self;
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
