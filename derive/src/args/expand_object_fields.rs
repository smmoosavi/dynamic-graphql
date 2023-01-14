use crate::utils::attributes::Attributes;
use crate::utils::common::{CommonArg, CommonField, CommonMethod, GetArgs, GetFields, GetGenerics};
use crate::utils::deprecation::Deprecation;
use crate::utils::impl_block::{BaseFnArg, BaseItemImpl, BaseMethod};
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;
use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Generics;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsArgAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub ctx: bool,
}

impl Attributes for ExpandObjectFieldsArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ExpandObjectFieldsArgContext {
    pub rename_args: Option<RenameRule>,
}

from_fn_arg!(ExpandObjectFieldsArg,
    WithAttributes<
        ExpandObjectFieldsArgAttrs,
        WithIndex<WithContext<ExpandObjectFieldsArgContext, BaseFnArg>>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsMethodAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

impl Attributes for ExpandObjectFieldsMethodAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ExpandObjectFieldsMethodContext {
    pub rename_args: Option<RenameRule>,
    pub rename_fields: Option<RenameRule>,
}

from_method!(
    ExpandObjectFieldsMethod,
    WithAttributes<
        WithDoc<ExpandObjectFieldsMethodAttrs>,
        WithContext<ExpandObjectFieldsMethodContext, BaseMethod<ExpandObjectFieldsArg>>,
    >,
    inner = args,
);

impl MakeContext<ExpandObjectFieldsArgContext> for ExpandObjectFieldsMethod {
    fn make_context(&self) -> ExpandObjectFieldsArgContext {
        ExpandObjectFieldsArgContext {
            rename_args: self.attrs.rename_args.or(self.ctx.rename_args),
        }
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsAttrs {
    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,
}

impl Attributes for ExpandObjectFieldsAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

from_item_impl!(
    ExpandObjectFields,
    WithAttributes<
        WithDoc<ExpandObjectFieldsAttrs>,
        BaseItemImpl<ExpandObjectFieldsMethod, Generics>,
    >,
    ctx,
);

impl MakeContext<ExpandObjectFieldsMethodContext> for ExpandObjectFields {
    fn make_context(&self) -> ExpandObjectFieldsMethodContext {
        ExpandObjectFieldsMethodContext {
            rename_args: self.attrs.rename_args,
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonField for ExpandObjectFieldsMethod {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        self.output_type.as_ref().ok_or_else(|| {
            darling::Error::custom("Field must have return type").with_span(&self.ident)
        })
    }

    fn get_skip(&self) -> bool {
        self.attrs.skip
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        Ok(self.attrs.deprecation.clone())
    }
    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_fields.as_ref()
    }
    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs
            .rename_args
            .as_ref()
            .or(self.ctx.rename_args.as_ref())
    }
}

impl CommonArg for ExpandObjectFieldsArg {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_arg(&self) -> &BaseFnArg {
        self
    }

    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_args.as_ref()
    }

    fn is_marked_as_ctx(&self) -> bool {
        self.attrs.ctx
    }
}

impl CommonMethod for ExpandObjectFieldsMethod {
    fn is_async(&self) -> bool {
        self.asyncness
    }
}

impl GetArgs<ExpandObjectFieldsArg> for ExpandObjectFieldsMethod {
    fn get_args(&self) -> darling::Result<&Vec<ExpandObjectFieldsArg>> {
        Ok(&self.args)
    }
}

impl GetFields<ExpandObjectFieldsMethod> for ExpandObjectFields {
    fn get_fields(&self) -> darling::Result<&Vec<ExpandObjectFieldsMethod>> {
        Ok(&self.methods)
    }
}

impl GetGenerics for ExpandObjectFields {
    fn get_generics(&self) -> &syn::Generics {
        &self.generics
    }
}

impl ToTokens for ExpandObjectFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {});
    }
}
