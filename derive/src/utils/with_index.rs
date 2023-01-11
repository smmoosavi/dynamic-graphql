use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::FromFnArg;
use std::ops::Deref;
use syn::FnArg;

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

impl<A: FromFnArg> FromFnArg for WithIndex<A> {
    fn from_fn_arg(arg: &mut FnArg, index: usize) -> GeneratorResult<Self> {
        let inner = A::from_fn_arg(arg, index)?;
        Ok(WithIndex { index, inner })
    }
}
