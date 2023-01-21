use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::utils::common::{CommonInterfacable, CommonObject};
use crate::utils::crate_name::get_crate_name;
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
            let ident = syn::Ident::new(interface, interface.span());
            quote! {
                let object = object.implement(<#ident as #crate_name::Interface>::NAME);
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

    let (_, ty_generics, _) = object.get_generics()?.split_for_impl();

    let implements: Vec<TokenStream> = implement
        .iter()
        .map(|interface| {
            let ident = syn::Ident::new(interface, interface.span());
            quote! {
                let registry = registry.register::<#ident<#object_type #ty_generics>>();
            }
        })
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
            let ident = syn::Ident::new(interface, interface.span());
            let mark = quote!(<#ident as #crate_name::Interface>::MARK);
            quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            }
        })
        .collect();

    let mark_implement: Vec<_> = object
        .get_implement()
        .iter()
        .map(|interface| {
            let ident = syn::Ident::new(interface, interface.span());
            let mark = quote!(<#ident as #crate_name::Interface>::MARK);
            quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            }
        })
        .collect();

    Ok(quote! {
        #(#mark_as)*
        #(#mark_with)*
        #(#mark_implement)*
    })
}
