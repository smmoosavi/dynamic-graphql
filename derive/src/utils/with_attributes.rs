use std::ops::Deref;
use std::ops::DerefMut;

use darling::FromAttributes;
use darling::FromDeriveInput;
use darling::FromField;
use darling::FromVariant;
use syn::FnArg;

use crate::utils::attributes::Attributes;
use crate::utils::attributes::CleanAttributes;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::impl_block::FromFnArg;
use crate::utils::impl_block::FromImplItemMethod;
use crate::utils::impl_block::FromItemTrait;
use crate::utils::impl_block::FromTraitItemMethod;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use crate::FromItemImpl;

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

impl<A: FromAttributes, D> DerefMut for WithAttributes<A, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

macro_rules! impl_for_with_attributes {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<A: FromAttributes, D: $trayt> $trayt for WithAttributes<A, D> {
            fn $method(input: &$syn) -> darling::Result<Self> {
                let attrs = A::from_attributes(&input.attrs)?;
                let data = D::$method(input)?;
                Ok(WithAttributes { attrs, inner: data })
            }
        }
    };
}

macro_rules! impl_mut_for_with_attributes {
    ($trayt:ident, $method:ident, $syn:path) => {
        impl<A: FromAttributes + Attributes, D: $trayt> $trayt for WithAttributes<A, D> {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                let inner = D::$method(input)?;
                let attrs = A::from_attributes(&input.attrs)?;
                A::clean_attributes(&mut input.attrs);
                Ok(WithAttributes { attrs, inner })
            }
        }
    };
}

impl_for_with_attributes!(FromDeriveInput, from_derive_input, syn::DeriveInput);
impl_for_with_attributes!(FromField, from_field, syn::Field);
impl_for_with_attributes!(FromVariant, from_variant, syn::Variant);

impl_mut_for_with_attributes!(
    FromImplItemMethod,
    from_impl_item_method,
    syn::ImplItemMethod
);
impl_mut_for_with_attributes!(FromItemImpl, from_item_impl, syn::ItemImpl);
impl_mut_for_with_attributes!(FromItemTrait, from_item_trait, syn::ItemTrait);
impl_mut_for_with_attributes!(
    FromTraitItemMethod,
    from_trait_item_method,
    syn::TraitItemMethod
);

impl<A: FromAttributes + Attributes, D: FromFnArg> FromFnArg for WithAttributes<A, D> {
    fn from_fn_arg(arg: &mut FnArg) -> darling::Result<Self> {
        let inner = D::from_fn_arg(arg)?;
        let base_attrs = BaseFnArg::get_attrs_mut(arg);
        let attrs = A::from_attributes(base_attrs)?;
        A::clean_attributes(base_attrs);
        Ok(WithAttributes { attrs, inner })
    }
}

impl<A: FromAttributes, D: SetIndex> SetIndex for WithAttributes<A, D> {
    fn with_index(self, index: usize) -> Self {
        WithAttributes {
            attrs: self.attrs,
            inner: D::with_index(self.inner, index),
        }
    }
}

impl<A: FromAttributes, D: SetContext> SetContext for WithAttributes<A, D> {
    type Context = D::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.inner.set_context(context)
    }
}
