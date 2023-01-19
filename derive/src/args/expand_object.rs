use darling::{FromAttributes, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

use crate::args::common;
use crate::args::common::{get_add_implement_code, get_interface_code};
use crate::utils::common::{CommonInterfacable, CommonObject};
use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceAttr;
use crate::utils::macros::*;
use crate::utils::type_utils::{get_owned_type, get_type_ident};
use crate::utils::with_attributes::WithAttributes;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectAttrs {
    #[darling(default, multiple)]
    pub mark_as: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub mark_with: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub implement: Vec<InterfaceAttr>,
}

from_derive_input!(ExpandObject, WithAttributes<ExpandObjectAttrs, NewtypeStruct<TupleField, Generics>>);

impl CommonObject for ExpandObject {
    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<syn::Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(None)
    }
}

impl CommonInterfacable for ExpandObject {
    fn get_mark_as(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.mark_as
    }

    fn get_mark_with(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.mark_with
    }

    fn get_implement(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.implement
    }
}

fn impl_expand_object(object: &ExpandObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    let target = get_owned_type(&object.data.ty);
    let name = object.ident.to_string();
    let type_ident = get_type_ident(&object.data.ty)?;

    // parse generic
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #create_name::ExpandObject for #object_ident #ty_generics #where_clause {
            const NAME: &'static str = #name;
            type Target = #target;
        }
        impl #impl_generics #create_name::InterfaceTarget for #object_ident #ty_generics #where_clause {
            const TARGET: &'static str = <#type_ident as #create_name::InterfaceTarget>::TARGET;
        }
    })
}

fn impl_parent(object: &ExpandObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
                fn parent(&self) -> &'a <Self as #create_name::ExpandObject>::Target {
                    self.0
                }
        }
    })
}

fn impl_from(object: &ExpandObject) -> darling::Result<TokenStream> {
    let object_ident = &object.ident;
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();
    let inner_type = &object.data.ty;

    Ok(quote! {
        impl #impl_generics From<#inner_type> for #object_ident #ty_generics #where_clause {
            fn from(target: #inner_type) -> Self {
                Self(target)
            }
        }
    })
}

fn impl_register_interface(object: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = object.get_ident();
    let add_interfaces = get_interface_code(object)?;
    let implement = get_add_implement_code(object, object.get_implement())?;
    let generics = common::add_static_to_types_in_where_clause(object.get_generics()?);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __register_interface(registry: #create_name::Registry) -> #create_name::Registry {
                #implement
                let registry = registry.update_object(
                    <<Self as #create_name::ExpandObject>::Target as #create_name::Object>::NAME,
                    <Self as #create_name::ExpandObject>::NAME,
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

fn impl_register_fns_trait(object: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = object.get_ident();
    let register_interface = impl_register_interface(object).into_token_stream();

    let generics = common::add_static_to_types_in_where_clause(object.get_generics()?);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let q = quote! {
        #register_interface
        impl #impl_generics #create_name::RegisterFns for #object_ident #ty_generics #where_clause {
            const REGISTER_FNS: &'static [fn (registry: #create_name::Registry) -> #create_name::Registry] = &[Self::__register_interface];
        }
    };

    Ok(q)
}

impl ToTokens for ExpandObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_expand_object = impl_expand_object(self).into_token_stream();
        let impl_parent = impl_parent(self).into_token_stream();
        let impl_from = impl_from(self).into_token_stream();
        let impl_register_fns_trait = impl_register_fns_trait(self).unwrap();
        tokens.extend(quote! {
            #impl_expand_object
            #impl_from
            #impl_parent
            #impl_register_fns_trait
        });
    }
}
