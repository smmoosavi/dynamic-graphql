use crate::utils::attributes::Attributes;
use crate::utils::docs_utils::get_rustdoc;
use darling::FromAttributes;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct WithDoc<D> {
    pub doc: Option<String>,
    pub inner: D,
}

impl<D> Deref for WithDoc<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D> AsRef<D> for WithDoc<D> {
    fn as_ref(&self) -> &D {
        &self.inner
    }
}

impl<D: FromAttributes> FromAttributes for WithDoc<D> {
    fn from_attributes(items: &[syn::Attribute]) -> darling::Result<Self> {
        let doc = get_rustdoc(items)?;
        let inner = D::from_attributes(items)?;
        Ok(WithDoc { doc, inner })
    }
}

impl<A: Attributes> Attributes for WithDoc<A> {
    const ATTRIBUTES: &'static [&'static str] = A::ATTRIBUTES;
}
