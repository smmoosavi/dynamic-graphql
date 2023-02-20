use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::common::CommonInterfaceAttrs;
use crate::utils::common::CommonObject;
use crate::utils::crate_name::get_crate_name;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceImplAttr;

pub fn get_register_interface_code(
    obj: &impl CommonInterfaceAttrs,
) -> darling::Result<TokenStream> {
    let mut paths = Vec::new();
    obj.get_marks().iter().for_each(|mark| {
        paths.push(mark.path.clone());
    });
    obj.get_impls().iter().for_each(|impl_attr| {
        paths.push(impl_attr.path.clone());
    });
    let codes: Vec<_> = paths
        .iter()
        .map(|path| {
            quote! {
                let registry = registry.register::<dyn #path>();
            }
        })
        .collect();
    Ok(quote! {
        #(#codes)*
    })
}

pub fn get_interface_mark_code(obj: &impl CommonInterfaceAttrs) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let implements: Vec<TokenStream> = obj
        .get_marks()
        .iter()
        .map(|interface| {
            let path = &interface.path;
            quote! {
                let object = object.implement(<dyn #path as #crate_name::internal::Interface>::get_interface_type_name());
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
    let crate_name = get_crate_name();
    let object_type = object.get_type()?;

    let (_, ty_generics, _) = object.get_generics()?.split_for_impl();

    let implements: Vec<TokenStream> = implement
        .iter()
        .map(|interface| {
            let path = &interface.path;
            let ty = quote!(#crate_name::Instance<dyn #path, #object_type #ty_generics>);
            Ok(quote! {
                let registry = registry.register::<#ty>();
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
            quote! {
                impl #crate_name::internal::InterfaceMark<dyn #path> for #object_ident {}
            }
        })
        .collect();

    let mark_implement: Vec<_> = object
        .get_impls()
        .iter()
        .map(|interface| {
            let path = &interface.path;
            quote! {
                impl #crate_name::internal::InterfaceMark<dyn #path> for #object_ident {}
            }
        })
        .collect();

    Ok(quote! {
        #(#marks)*
        #(#mark_implement)*
    })
}
