use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Generics;

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
use crate::utils::type_utils::{get_type_path, remove_path_generics};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;

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
    pub expand_ty: Option<syn::Type>,
}

from_impl_item_method!(
    ExpandObjectFieldsMethod,
    WithAttributes<
        WithDoc<ExpandObjectFieldsMethodAttrs>,
        WithIndex<WithContext<ExpandObjectFieldsMethodContext, BaseMethod<ExpandObjectFieldsArg>>>,
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
            expand_ty: Some(self.ty.clone()),
        }
    }
}

impl CommonObject for ExpandObjectFields {
    fn get_name(&self) -> Option<&str> {
        unreachable!("ResolvedObjectFields does not have a name");
    }

    fn should_impl_type_name(&self) -> bool {
        false
    }

    fn get_ident(&self) -> &syn::Ident {
        unreachable!("ResolvedObjectFields does not have an ident");
    }

    fn get_type(&self) -> darling::Result<syn::Path> {
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

impl ArgImplementor for ExpandObjectFieldsArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        let crate_name = get_crate_name();
        let arg_ident = common::get_arg_ident(self);
        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<<Self as #crate_name::internal::ParentType>::Type>()?.into();
            let #arg_ident = &parent;
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

impl FieldImplementor for ExpandObjectFieldsMethod {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }
    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        let ty = self.ctx.expand_ty.as_ref().unwrap_or_else(|| {
            unreachable!("ExpandObjectFieldsMethodContext::expand_ty must be set")
        });
        let type_path = remove_path_generics(get_type_path(ty)?);
        execute_code(&type_path, self)
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        common::resolve_value_code()
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
        let field_var_ident = get_field_var_ident(self.index, &self.ident);

        Ok(quote! {
            let #field_var_ident = field;
        })
    }
}

fn get_field_var_ident(index: usize, ident: &syn::Ident) -> Ident {
    Ident::new(&format!("__field_{}", index), ident.span())
}

fn execute_code<F, A>(type_path: &syn::Path, method: &F) -> darling::Result<TokenStream>
where
    F: CommonMethod + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    let field_ident = method.get_ident()?;

    let args = common::get_args_usage(method)?;

    if method.is_async() {
        Ok(quote! {
            let value = #type_path::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = #type_path::#field_ident(#args);
        })
    }
}

fn use_field_code(index: usize, method: &ExpandObjectFieldsMethod) -> darling::Result<TokenStream> {
    let field_var_ident = get_field_var_ident(index, &method.ident);

    Ok(quote! {
        let object = object.field(#field_var_ident);
    })
}

fn use_fields_code(expand: &ExpandObjectFields) -> darling::Result<TokenStream> {
    Ok(expand
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .enumerate()
        .map(|(index, method)| use_field_code(index, method).into_token_stream())
        .collect())
}

fn impl_register(expand: &ExpandObjectFields) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let (impl_generics, _, where_clause) = expand.generics.split_for_impl();
    let ty = get_type_path(&expand.ty)?;

    let register_nested_types = common::get_nested_type_register_code(expand).into_token_stream();

    let define_fields = common::get_define_fields_code(expand).into_token_stream();

    let use_fields = use_fields_code(expand).into_token_stream();

    let register_fns = common::call_register_fns();
    Ok(quote! {
        impl #impl_generics #crate_name::internal::Register for #ty #where_clause {
            fn register(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry {

                #register_nested_types

                #register_fns

                #define_fields
                registry.update_object(
                    <<Self as #crate_name::internal::ParentType>::Type as #crate_name::internal::Object>::get_object_type_name().as_ref(),
                    <Self as #crate_name::internal::ExpandObject>::get_expand_object_name().as_ref(),
                    |object| {
                        #use_fields
                        object
                    },
                )
            }
        }
    })
}

impl ToTokens for ExpandObjectFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_register
        });
    }
}
