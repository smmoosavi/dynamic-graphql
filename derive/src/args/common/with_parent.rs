use crate::args::common::{CommonArg, CommonField, CommonObject};
use crate::utils::deprecation::Deprecation;
use crate::utils::error::{GeneratorResult, WithSpan};
use crate::utils::rename_rule::RenameRule;

pub struct WithParent<'f, 'p, F, P> {
    inner: &'f F,
    parent: &'p P,
}

impl<'f, 'p, F, P> WithParent<'f, 'p, F, P> {
    pub fn new(field: &'f F, parent: &'p P) -> Self {
        Self {
            inner: field,
            parent,
        }
    }
}

impl<'f, F, P> CommonField for WithParent<'f, '_, F, P>
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

impl<'f, F, P> CommonArg for WithParent<'f, '_, F, P>
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
