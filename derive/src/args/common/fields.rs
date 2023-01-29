use proc_macro2::TokenStream;
use quote::quote;

use crate::args::common::{ArgImplementor, FieldImplementor};
use crate::utils::common::{CommonField, GetArgs};
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::rename_rule::calc_field_name;
use crate::utils::type_utils::get_owned_type;

pub fn get_field_type(field: &impl CommonField) -> darling::Result<&syn::Type> {
    let ty = field.get_type()?;
    let owned_type = get_owned_type(ty);
    Ok(owned_type)
}

pub fn get_args_definition<F, A>(field: &F) -> darling::Result<TokenStream>
where
    F: GetArgs<A>,
    A: ArgImplementor,
{
    let args = field.get_args()?;
    Ok(args
        .iter()
        .map(|arg| ArgImplementor::get_arg_definition(arg).into_token_stream())
        .collect())
}

pub fn get_args_usage<F, A>(field: &F) -> darling::Result<TokenStream>
where
    F: GetArgs<A>,
    A: ArgImplementor,
{
    let args = field.get_args()?;
    Ok(args
        .iter()
        .map(|arg| ArgImplementor::get_arg_usage(arg).into_token_stream())
        .collect())
}

pub fn field_deprecation_code(deprecation: &impl CommonField) -> darling::Result<TokenStream> {
    let deprecation = deprecation.get_deprecation()?;
    // todo get "field" from input
    match deprecation {
        Deprecation::NoDeprecated => Ok(quote! {}),
        Deprecation::Deprecated { reason: None } => Ok(quote! {
            let field = field.deprecation(None);
        }),
        Deprecation::Deprecated {
            reason: Some(ref reason),
        } => Ok(quote! {
            let field = field.deprecation(Some(#reason));
        }),
    }
}

pub fn field_description(field: &impl CommonField) -> darling::Result<TokenStream> {
    let doc = field.get_doc()?;
    // todo get "field" from input
    if let Some(doc) = doc {
        Ok(quote! {
            let field = field.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}

pub fn get_field_name(field: &impl CommonField) -> darling::Result<String> {
    Ok(calc_field_name(
        field.get_name(),
        &field.get_ident()?.to_string(),
        field.get_field_rename_rule(),
    ))
}

pub fn define_field<F, A>(method: &F) -> darling::Result<TokenStream>
where
    F: FieldImplementor + GetArgs<A>,
    A: ArgImplementor,
{
    let crate_name = get_crate_name();

    let field_name = get_field_name(method)?;
    let field_type = get_field_type(method)?;
    let graphql_args_definition = get_args_definition(method)?;
    let execute = method.get_execute_code()?;
    let resolve = method.get_resolve_code()?;
    Ok(quote! {
        let field = #crate_name::dynamic::Field::new(#field_name, <#field_type as #crate_name::GetOutputTypeRef>::get_output_type_ref(), |ctx| {
            #crate_name::dynamic::FieldFuture::new(async move {
                #graphql_args_definition
                #execute
                #resolve
            })
        });
    })
}

pub fn build_field<F, A>(method: &F) -> darling::Result<TokenStream>
where
    F: FieldImplementor + GetArgs<A>,
    A: ArgImplementor,
{
    let define_field = method.define_field()?;
    let argument_definitions = method.get_field_argument_definition()?;
    let description = method.get_field_description_code()?;
    let deprecation = method.get_field_deprecation_code()?;
    let field_usage = method.get_field_usage_code()?;

    Ok(quote! {
        #define_field
        #argument_definitions
        #description
        #deprecation
        #field_usage
    })
}

pub fn resolve_value_code(_ty: &syn::Type) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();

    Ok(quote! {
        #crate_name::Resolve::resolve(value, &ctx)
    })
}
