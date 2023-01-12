use crate::utils::attributes::{Attributes, CleanAttributes};
use crate::utils::error::GeneratorResult;
use crate::utils::impl_block::{BaseFnArg, FromFnArg, FromMethod};
use crate::utils::with_index::SetIndex;
use crate::FromItemImpl;
use darling::{FromAttributes, FromDeriveInput, FromField, FromVariant};
use std::ops::Deref;
use syn::{FnArg, ImplItemMethod, ItemImpl};

#[derive(Clone, Debug)]
pub struct WithAttributes<A: FromAttributes, D> {
    pub attrs: A,
    pub inner: D,
}

// deref
impl<A: FromAttributes, D> Deref for WithAttributes<A, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<A: FromAttributes, D: FromDeriveInput> FromDeriveInput for WithAttributes<A, D> {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let attrs = A::from_attributes(&input.attrs)?;
        let data = D::from_derive_input(input)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}

impl<A: FromAttributes, D: FromField> FromField for WithAttributes<A, D> {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let attrs = A::from_attributes(&field.attrs)?;
        let data = D::from_field(field)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}

impl<A: FromAttributes, D: FromVariant> FromVariant for WithAttributes<A, D> {
    fn from_variant(variant: &syn::Variant) -> darling::Result<Self> {
        let attrs = A::from_attributes(&variant.attrs)?;
        let data = D::from_variant(variant)?;
        Ok(WithAttributes { attrs, inner: data })
    }
}

impl<A: FromAttributes + Attributes, D: FromFnArg> FromFnArg for WithAttributes<A, D> {
    fn from_fn_arg(arg: &mut FnArg) -> GeneratorResult<Self> {
        let inner = D::from_fn_arg(arg)?;
        let base_attrs = BaseFnArg::get_attrs_mut(arg);
        let attrs = A::from_attributes(base_attrs)?;
        A::clean_attributes(base_attrs);
        Ok(WithAttributes { attrs, inner })
    }
}
impl<A: FromAttributes + Attributes, D: FromMethod> FromMethod for WithAttributes<A, D> {
    fn from_method(method: &mut ImplItemMethod) -> GeneratorResult<Self> {
        let inner = D::from_method(method)?;
        let attrs = A::from_attributes(&method.attrs)?;
        A::clean_attributes(&mut method.attrs);
        Ok(WithAttributes { attrs, inner })
    }
}
impl<A: FromAttributes + Attributes, D: FromItemImpl> FromItemImpl for WithAttributes<A, D> {
    fn from_item_impl(item_impl: &mut ItemImpl) -> GeneratorResult<Self> {
        let inner = D::from_item_impl(item_impl)?;
        let attrs = A::from_attributes(&item_impl.attrs)?;
        A::clean_attributes(&mut item_impl.attrs);
        Ok(WithAttributes { attrs, inner })
    }
}

impl<A: FromAttributes, D: SetIndex> SetIndex for WithAttributes<A, D> {
    type Output = WithAttributes<A, D::Output>;
    fn with_index(self, index: usize) -> Self::Output {
        WithAttributes {
            attrs: self.attrs,
            inner: D::with_index(self.inner, index),
        }
    }
}
