use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::args::common::{generics, replace_generic_lifetime_with_static};
use crate::utils::common::{CommonInterfacable, CommonObject};
use crate::utils::crate_name::get_create_name;
use crate::utils::interface_attr::InterfaceAttr;

pub fn get_interface_code(obj: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let mark_with_code = get_add_mark_with_code(obj.get_mark_with())?;
    let mark_as_code = get_add_mark_as_code(obj.get_mark_as())?;

    Ok(quote! {
        #mark_with_code
        #mark_as_code
    })
}

fn get_add_mark_with_code(mark_with: &[InterfaceAttr]) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let implements: Vec<TokenStream> = mark_with
        .iter()
        .map(|interface| {
            let ident = syn::Ident::new(interface, interface.span());
            quote! {
                let object = object.implement(<#ident as #create_name::Interface>::NAME);
            }
        })
        .collect();
    Ok(quote! {
        #(#implements)*
    })
}

fn get_add_mark_as_code(mark_as: &[InterfaceAttr]) -> darling::Result<TokenStream> {
    let implements: Vec<TokenStream> = mark_as
        .iter()
        .map(|interface| {
            let name = interface.to_string();
            quote! {
                let object = object.implement(#name);
            }
        })
        .collect();
    Ok(quote! {
        #(#implements)*
    })
}

pub fn get_add_implement_code(
    object: &impl CommonObject,
    implement: &[InterfaceAttr],
) -> darling::Result<TokenStream> {
    if implement.is_empty() {
        return Ok(quote! {});
    }
    let object_type = object.get_type()?;

    let object_type = generics::replace_path_lifetime_with_static(&object_type);

    let static_generics = replace_generic_lifetime_with_static(object.get_generics()?);
    let (_, static_ty_generics, _) = static_generics.split_for_impl();

    let implements: Vec<TokenStream> = implement
        .iter()
        .map(|interface| {
            let ident = syn::Ident::new(interface, interface.span());
            quote! {
                let registry = registry.register::<#ident<#object_type #static_ty_generics>>();
            }
        })
        .collect();
    Ok(quote! {
        #(#implements)*
    })
}
