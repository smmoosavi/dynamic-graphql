use crate::utils::attributes::{Attributes, CleanAttributes};
use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::{BaseFnArg, FromFnArg, FromItemImpl, FromMethod};
use darling::FromAttributes;
use std::ops::Deref;
use syn::{FnArg, ImplItemMethod, ItemImpl};

#[derive(Debug, Clone)]
pub struct WithCleanAttributes<A: FromAttributes + Attributes, D> {
    pub attrs: A,
    pub inner: D,
}

impl<A: FromAttributes + Attributes, D> Deref for WithCleanAttributes<A, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<A: FromAttributes + Attributes, D: FromFnArg> FromFnArg for WithCleanAttributes<A, D> {
    fn from_fn_arg(arg: &mut FnArg) -> GeneratorResult<Self> {
        let inner = D::from_fn_arg(arg)?;
        let base_attrs = BaseFnArg::get_attrs_mut(arg);
        let attrs = A::from_attributes(base_attrs)?;
        A::clean_attributes(base_attrs);
        Ok(WithCleanAttributes { attrs, inner })
    }
}
impl<A: FromAttributes + Attributes, D: FromMethod> FromMethod for WithCleanAttributes<A, D> {
    fn from_method(method: &mut ImplItemMethod) -> GeneratorResult<Self> {
        let inner = D::from_method(method)?;
        let attrs = A::from_attributes(&method.attrs)?;
        A::clean_attributes(&mut method.attrs);
        Ok(WithCleanAttributes { attrs, inner })
    }
}
impl<A: FromAttributes + Attributes, D: FromItemImpl> FromItemImpl for WithCleanAttributes<A, D> {
    fn from_item_impl(item_impl: &mut ItemImpl) -> GeneratorResult<Self> {
        let inner = D::from_item_impl(item_impl)?;
        let attrs = A::from_attributes(&item_impl.attrs)?;
        A::clean_attributes(&mut item_impl.attrs);
        Ok(WithCleanAttributes { attrs, inner })
    }
}
