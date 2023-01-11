use crate::utils::common::{CommonArg, CommonField, CommonObject};
use crate::utils::deprecation::Deprecation;
use crate::utils::error::{GeneratorResult, WithSpan};
use crate::utils::rename_rule::RenameRule;
use std::ops::Deref;

pub trait WithParent<'i, 'p, P>
where
    Self: 'i,
    P: 'p,
{
    type Output;
    fn with_parent(&'i self, parent: &'p P) -> Self::Output;
}

impl<'i, 'p, I, P> WithParent<'i, 'p, P> for I
where
    I: 'i,
    P: 'p,
    I: CommonField,
    P: CommonObject,
{
    type Output = WithParentInner<'i, 'p, I, P>;

    fn with_parent(&'i self, parent: &'p P) -> Self::Output {
        WithParentInner {
            inner: self,
            parent,
        }
    }
}

pub struct WithParentInner<'i, 'p, I, P> {
    inner: &'i I,
    parent: &'p P,
}

impl<'i, 'p, I, P> Deref for WithParentInner<'i, 'p, I, P>
where
    I: 'i,
    P: 'p,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'f, F, P> CommonField for WithParentInner<'f, '_, F, P>
where
    F: CommonField,
    P: CommonObject,
{
    fn get_name(&self) -> Option<&str> {
        self.inner.get_name()
    }

    fn get_ident(&self) -> GeneratorResult<&syn::Ident> {
        self.inner.get_ident().with_span(&self.parent.get_ident())
    }

    fn get_type(&self) -> GeneratorResult<&syn::Type> {
        self.inner.get_type()
    }

    fn get_skip(&self) -> bool {
        self.inner.get_skip()
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
        self.inner.get_doc()
    }

    fn get_deprecation(&self) -> GeneratorResult<Deprecation> {
        self.inner.get_deprecation()
    }

    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.inner
            .get_field_rename_rule()
            .or_else(|| self.parent.get_fields_rename_rule())
    }
    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        self.inner
            .get_args_rename_rule()
            .or_else(|| self.parent.get_args_rename_rule())
    }
}

impl<'f, F, P> CommonArg for WithParentInner<'f, '_, F, P>
where
    F: CommonArg,
    P: CommonField,
{
    fn get_name(&self) -> Option<&str> {
        self.inner.get_name()
    }

    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        self.inner
            .get_arg_rename_rule()
            .or_else(|| self.parent.get_args_rename_rule())
    }
}
