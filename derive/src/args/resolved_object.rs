use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{Generics, Path};

use crate::args::common;
use crate::args::common::{
    get_add_implement_code, get_interface_mark_code, get_register_interface_code,
};
use crate::utils::common::{CommonInterfaceAttrs, CommonObject};
use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::BaseStruct;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::{InterfaceImplAttr, InterfaceMarkAttr};
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectAttrs {
    #[darling(default)]
    pub root: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,

    #[darling(default, multiple)]
    #[darling(rename = "mark")]
    pub marks: Vec<InterfaceMarkAttr>,

    #[darling(default, multiple)]
    #[darling(rename = "impl")]
    pub impls: Vec<InterfaceImplAttr>,
}

from_derive_input!(
    ResolvedObject,
    WithAttributes<WithDoc<ResolvedObjectAttrs>, BaseStruct<(), Generics>>,
);

impl CommonObject for ResolvedObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn should_impl_type_name(&self) -> bool {
        !self.attrs.type_name
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
}

impl CommonInterfaceAttrs for ResolvedObject {
    fn get_marks(&self) -> &Vec<InterfaceMarkAttr> {
        &self.attrs.marks
    }

    fn get_impls(&self) -> &Vec<InterfaceImplAttr> {
        &self.attrs.impls
    }
}

fn impl_register_interface(object: &impl CommonInterfaceAttrs) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let register_interface_code = get_register_interface_code(object)?;
    let add_interfaces = get_interface_mark_code(object)?;
    let implement = get_add_implement_code(object, object.get_impls())?;
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __register_interface(registry: #crate_name::Registry) -> #crate_name::Registry {
                #register_interface_code
                #implement
                let registry = registry.update_object(
                    <Self as #crate_name::Object>::get_object_type_name().as_str(),
                    <Self as #crate_name::Object>::get_object_type_name().as_str(),
                    |object| {
                        #add_interfaces
                        object
                    },
                );
                registry
            }
        }
    })
}

fn impl_register_root(object: &ResolvedObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    let root = if object.attrs.root {
        let crate_name = get_crate_name();
        Some(quote! {
            let registry = registry.set_root(<Self as #crate_name::Object>::get_object_type_name().as_str());
        })
    } else {
        None
    };

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __register_root(registry: #crate_name::Registry) -> #crate_name::Registry {
                #root
                registry
            }
        }
    })
}

fn impl_graphql_doc_fn(object: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    let doc = object
        .get_doc()
        .map(|doc| {
            if let Some(doc) = doc {
                let crate_name = get_crate_name();
                Some(quote! {

                        let registry = registry.update_object(
                            <Self as #crate_name::Object>::get_object_type_name().as_str(),
                            <Self as #crate_name::Object>::get_object_type_name().as_str(),
                            |object| {
                                object.description(#doc)
                            },
                        );

                })
            } else {
                None
            }
        })
        .into_token_stream();

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __register_doc(registry: #crate_name::Registry) -> #crate_name::Registry {
                #doc
                registry
            }
        }
    })
}

fn impl_registers_fn(object: &ResolvedObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    let register_attr = &object.attrs.registers;

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __registers(registry: #crate_name::Registry) -> #crate_name::Registry {
                #( #register_attr )*
                registry
            }
        }
    })
}

fn impl_register_fns_trait(obj: &impl CommonInterfaceAttrs) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = obj.get_ident();
    let generics = obj.get_generics()?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let turbofish_generics = ty_generics.as_turbofish();

    Ok(quote! {
        impl #impl_generics #crate_name::RegisterFns for #object_ident #ty_generics #where_clause {
            const REGISTER_FNS: &'static [fn (registry: #crate_name::Registry) -> #crate_name::Registry] = &[
                #object_ident #turbofish_generics ::__register_interface,
                #object_ident #turbofish_generics ::__register_root,
                #object_ident #turbofish_generics ::__register_doc,
                #object_ident #turbofish_generics ::__registers,
            ];
        }
    })
}

impl ToTokens for ResolvedObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
        let impl_graphql_doc = impl_graphql_doc_fn(self).into_token_stream();
        let register_interface = impl_register_interface(self).into_token_stream();
        let register_root = impl_register_root(self).into_token_stream();
        let impl_register_extras = impl_register_fns_trait(self).into_token_stream();
        let impl_interface_mark = common::impl_interface_mark(self).into_token_stream();
        let impl_registers_fn = impl_registers_fn(self).into_token_stream();

        tokens.extend(quote! {
            #impl_registers_fn
            #impl_object
            #impl_interface_mark
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_graphql_doc
            #register_interface
            #register_root
            #impl_register_extras
        });
    }
}
