use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::common::{CommonInterfacable, CommonObject};
use crate::utils::crate_name::get_crate_name;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceAttr;
use crate::utils::interface_hash::get_interface_hash;

pub fn get_interface_code(obj: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let mark_with_code = get_add_mark_with_code(obj.get_mark_with())?;
    let mark_as_code = get_add_mark_as_code(obj.get_mark_as())?;

    Ok(quote! {
        #mark_with_code
        #mark_as_code
    })
}

fn get_add_mark_with_code(mark_with: &[InterfaceAttr]) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let implements: Vec<TokenStream> = mark_with
        .iter()
        .map(|interface| {
            let path = interface.to_path()?;
            Ok(quote! {
                let object = object.implement(<#path as #crate_name::Interface>::NAME);
            })
        })
        .map(|x| x.into_token_stream())
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

    let (_, ty_generics, _) = object.get_generics()?.split_for_impl();

    let implements: Vec<TokenStream> = implement
        .iter()
        .map(|interface| {
            let path = interface.to_path()?;
            Ok(quote! {
                let registry = registry.register::<#path<#object_type #ty_generics>>();
            })
        })
        .map(|x| x.into_token_stream())
        .collect();
    Ok(quote! {
        #(#implements)*
    })
}

pub fn impl_interface_mark(object: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let mark_as: Vec<_> = object
        .get_mark_as()
        .iter()
        .map(|interface| {
            let name = interface.to_string();
            let mark = get_interface_hash(&name);
            quote! {
                impl #crate_name::InterfaceMark<#mark> for #object_ident {}
            }
        })
        .collect();

    let mark_with: Vec<_> = object
        .get_mark_with()
        .iter()
        .map(|interface| {
            let path = interface.to_path()?;
            let mark = quote!(<#path as #crate_name::Interface>::MARK);
            Ok(quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            })
        })
        .map(|x| x.into_token_stream())
        .collect();

    let mark_implement: Vec<_> = object
        .get_implement()
        .iter()
        .map(|interface| {
            let ident = interface.to_path()?;
            let mark = quote!(<#ident as #crate_name::Interface>::MARK);
            Ok(quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            })
        })
        .map(|x| x.into_token_stream())
        .collect();

    Ok(quote! {
        #(#mark_as)*
        #(#mark_with)*
        #(#mark_implement)*
    })
}
