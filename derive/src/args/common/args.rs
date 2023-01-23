use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::utils::common::CommonArg;
use crate::utils::crate_name::get_crate_name;
use crate::utils::impl_block::{BaseFnArg, TypedArg};
use crate::utils::rename_rule::calc_arg_name;
use crate::utils::type_utils::{get_owned_type, get_value_type, is_type_ref};

pub fn get_arg_ident(arg: &impl CommonArg) -> syn::Ident {
    syn::Ident::new(&format!("arg{}", arg.get_index()), arg.get_arg().span())
}

pub fn is_arg_ctx(arg: &impl CommonArg) -> bool {
    arg.is_marked_as_ctx()
        || matches!(arg.get_arg(), BaseFnArg::Typed(TypedArg{ref ident, ..}) if ident == "ctx" || ident == "_ctx")
}

pub fn get_self_arg_usage(arg: &impl CommonArg) -> darling::Result<TokenStream> {
    let arg_ident = get_arg_ident(arg);
    Ok(quote!(#arg_ident,))
}

pub fn get_typed_arg_usage(arg: &impl CommonArg) -> darling::Result<TokenStream> {
    let arg_ident = get_arg_ident(arg);
    let BaseFnArg::Typed(typed) = arg.get_arg() else {
        unreachable!("Expected typed argument");
    };
    let is_ctx = is_arg_ctx(arg);
    let is_owned = !is_type_ref(&typed.ty);
    if is_ctx || is_owned {
        Ok(quote!(#arg_ident,))
    } else {
        Ok(quote!(&#arg_ident,))
    }
}

pub fn get_argument_definition(arg: &impl CommonArg) -> TokenStream {
    if is_arg_ctx(arg) {
        return quote!();
    }
    let BaseFnArg::Typed(typed) = arg.get_arg() else {
        return quote!();
    };
    let crate_name = get_crate_name();
    let arg_name = calc_arg_name(
        arg.get_name(),
        &typed.ident.to_string(),
        arg.get_arg_rename_rule(),
    );
    let arg_type = get_owned_type(&typed.ty);

    quote! {
        let arg = #crate_name::dynamic::InputValue::new(#arg_name, <#arg_type as #crate_name::GetInputTypeRef>::get_input_type_ref());
        let field = field.argument(arg);
    }
}

pub fn get_argument_definitions(args: &[impl CommonArg]) -> darling::Result<TokenStream> {
    Ok(args.iter().map(get_argument_definition).collect())
}

pub fn get_typed_arg_definition(arg: &impl CommonArg) -> darling::Result<TokenStream> {
    let BaseFnArg::Typed(typed) = arg.get_arg() else {
        unreachable!("Expected typed argument");
    };
    let crate_name = get_crate_name();
    let arg_ident = get_arg_ident(arg);
    if is_arg_ctx(arg) {
        Ok(quote! {
            let #arg_ident = &ctx;
        })
    } else {
        let arg_name = calc_arg_name(
            arg.get_name(),
            &typed.ident.to_string(),
            arg.get_arg_rename_rule(),
        );
        let value_type = get_value_type(&typed.ty);
        match value_type {
            None => Ok(quote! {
                let #arg_ident = #crate_name::FromValue::from_value(ctx.args.try_get(#arg_name))?;
            }),
            Some(ty) => Ok(quote! {
                let #arg_ident: #ty = #crate_name::FromValue::from_value(ctx.args.try_get(#arg_name))?;
            }),
        }
    }
}
