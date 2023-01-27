use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::common::{CommonInterfaceAttrs, CommonObject};
use crate::utils::crate_name::get_crate_name;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceImplAttr;

pub fn get_interface_mark_code(obj: &impl CommonInterfaceAttrs) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let implements: Vec<TokenStream> = obj
        .get_marks()
        .iter()
        .map(|interface| {
            let path = &interface.path;
            quote! {
                let object = object.implement(<#path as #crate_name::Interface>::NAME);
            }
        })
        .collect();
    Ok(quote! {
        #(#implements)*
    })
}

pub fn get_add_implement_code(
    object: &impl CommonObject,
    implement: &[InterfaceImplAttr],
) -> darling::Result<TokenStream> {
    if implement.is_empty() {
        return Ok(quote! {});
    }
    let object_type = object.get_type()?;

    let (_, ty_generics, _) = object.get_generics()?.split_for_impl();

    let implements: Vec<TokenStream> = implement
        .iter()
        .map(|interface| {
            let path = &interface.path;
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

pub fn impl_interface_mark(object: &impl CommonInterfaceAttrs) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let marks: Vec<_> = object
        .get_marks()
        .iter()
        .map(|interface| {
            let path = &interface.path;
            let mark = quote!(<#path as #crate_name::Interface>::MARK);
            quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            }
        })
        .collect();

    let mark_implement: Vec<_> = object
        .get_impls()
        .iter()
        .map(|interface| {
            let ident = &interface.path;
            let mark = quote!(<#ident as #crate_name::Interface>::MARK);
            Ok(quote! {
                impl #crate_name::InterfaceMark<{#mark}> for #object_ident {}
            })
        })
        .map(|x| x.into_token_stream())
        .collect();

    Ok(quote! {
        #(#marks)*
        #(#mark_implement)*
    })
}
