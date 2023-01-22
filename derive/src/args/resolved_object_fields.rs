use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Generics, Path};

use crate::args::common;
use crate::args::common::{ArgImplementor, FieldImplementor};
use crate::utils::attributes::Attributes;
use crate::utils::common::{
    CommonArg, CommonField, CommonMethod, CommonObject, GetArgs, GetFields,
};
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::{BaseFnArg, BaseItemImpl, BaseMethod};
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::type_utils::{get_type_ident, get_type_path};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsArgAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub ctx: bool,
}

impl Attributes for ResolvedObjectFieldsArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ResolvedObjectFieldsArgContext {
    pub rename_args: Option<RenameRule>,
}

from_fn_arg!(ResolvedObjectFieldsArg,
    WithAttributes<
        ResolvedObjectFieldsArgAttrs,
        WithIndex<WithContext<ResolvedObjectFieldsArgContext, BaseFnArg>>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsMethodAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

impl Attributes for ResolvedObjectFieldsMethodAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ResolvedObjectFieldsMethodContext {
    pub rename_args: Option<RenameRule>,
    pub rename_fields: Option<RenameRule>,
}

from_impl_item_method!(
    ResolvedObjectFieldsMethod,
    WithAttributes<
        WithDoc<ResolvedObjectFieldsMethodAttrs>,
        WithContext<ResolvedObjectFieldsMethodContext, BaseMethod<ResolvedObjectFieldsArg>>,
    >,
    inner = args,
);

impl MakeContext<ResolvedObjectFieldsArgContext> for ResolvedObjectFieldsMethod {
    fn make_context(&self) -> ResolvedObjectFieldsArgContext {
        ResolvedObjectFieldsArgContext {
            rename_args: self.attrs.rename_args.or(self.ctx.rename_args),
        }
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsAttrs {
    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,
}

impl Attributes for ResolvedObjectFieldsAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

from_item_impl!(
    ResolvedObjectFields,
    WithAttributes<
        WithDoc<ResolvedObjectFieldsAttrs>,
        BaseItemImpl<ResolvedObjectFieldsMethod, Generics>,
    >,
    ctx,
);

impl MakeContext<ResolvedObjectFieldsMethodContext> for ResolvedObjectFields {
    fn make_context(&self) -> ResolvedObjectFieldsMethodContext {
        ResolvedObjectFieldsMethodContext {
            rename_args: self.attrs.rename_args,
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonObject for ResolvedObjectFields {
    fn get_name(&self) -> Option<&str> {
        unreachable!("ResolvedObjectFields does not have a name");
    }

    fn get_ident(&self) -> &Ident {
        unreachable!("ResolvedObjectFields does not have an ident");
    }

    fn get_type(&self) -> darling::Result<Path> {
        get_type_path(&self.ty).cloned()
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }

    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_fields.as_ref()
    }

    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_args.as_ref()
    }
}

impl CommonField for ResolvedObjectFieldsMethod {
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

impl CommonArg for ResolvedObjectFieldsArg {
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

impl CommonMethod for ResolvedObjectFieldsMethod {
    fn is_async(&self) -> bool {
        self.asyncness
    }
}

impl GetArgs<ResolvedObjectFieldsArg> for ResolvedObjectFieldsMethod {
    fn get_args(&self) -> darling::Result<&Vec<ResolvedObjectFieldsArg>> {
        Ok(&self.args)
    }
}

impl GetFields<ResolvedObjectFieldsMethod> for ResolvedObjectFields {
    fn get_fields(&self) -> darling::Result<&Vec<ResolvedObjectFieldsMethod>> {
        Ok(&self.methods)
    }
}

impl ArgImplementor for ResolvedObjectFieldsArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        let crate_name = get_crate_name();
        let arg_ident = common::get_arg_ident(self);

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<<Self as #crate_name::ParentType>::Type>()?.into();
            let #arg_ident = parent;
        })
    }

    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_definition(self)
    }

    fn get_self_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_self_arg_usage(self)
    }

    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_usage(self)
    }
}

impl FieldImplementor for ResolvedObjectFieldsMethod {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }
    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        execute_code(self)
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        let ty = self.get_type()?;
        common::resolve_value_code(ty)
    }

    fn get_field_argument_definition(&self) -> darling::Result<TokenStream> {
        common::get_argument_definitions(self.get_args()?)
    }

    fn get_field_description_code(&self) -> darling::Result<TokenStream> {
        common::field_description(self)
    }

    fn get_field_deprecation_code(&self) -> darling::Result<TokenStream> {
        common::field_deprecation_code(self)
    }

    fn get_field_usage_code(&self) -> darling::Result<TokenStream> {
        Ok(quote! {
            let object = object.field(field);
        })
    }
}

fn execute_code<F, A>(method: &F) -> darling::Result<TokenStream>
where
    F: CommonMethod + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    let field_ident = method.get_ident()?;

    let args = common::get_args_usage(method)?;

    if method.is_async() {
        Ok(quote! {
            let value = Self::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = Self::#field_ident(#args);
        })
    }
}

fn impl_object_description() -> TokenStream {
    let crate_name = get_crate_name();

    quote! {
        let object = <Self as #crate_name::GraphqlDoc>::DOC.iter().fold(object, |object, doc| {
            object.description(doc.to_owned())
        });
    }
}

fn get_define_fields_code<O, F, A>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: FieldImplementor + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    Ok(object
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .map(|method| common::build_field(method).into_token_stream())
        .collect())
}

fn impl_register(object: &ResolvedObjectFields) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ty = get_type_ident(&object.ty)?;
    let define_object = common::impl_define_object();
    let description = impl_object_description();
    let define_fields = get_define_fields_code(object)?;
    let register_object_code = common::register_object_code();
    let register_fns = common::call_register_fns();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::Register for #ty #ty_generics #where_clause {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {
                #define_object

                #define_fields

                #description

                #register_fns

                #register_object_code
            }
        }
    })
}

impl ToTokens for ResolvedObjectFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_register
        });
    }
}
