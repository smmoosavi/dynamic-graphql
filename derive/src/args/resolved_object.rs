use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{Generics, Path};

use crate::args::common;
use crate::args::common::{get_add_implement_code, get_interface_code};
use crate::utils::common::{CommonInterfacable, CommonObject};
use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::BaseStruct;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceAttr;
use crate::utils::macros::*;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default, multiple)]
    pub mark_as: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub mark_with: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub implement: Vec<InterfaceAttr>,
}

from_derive_input!(
    ResolvedObject,
    WithAttributes<WithDoc<ResolvedObjectAttrs>, BaseStruct<(), Generics>>,
);

impl CommonObject for ResolvedObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
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

impl CommonInterfacable for ResolvedObject {
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

fn impl_register_interface(object: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = object.get_ident();
    let add_interfaces = get_interface_code(object)?;
    let implement = get_add_implement_code(object, object.get_implement())?;
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __register_interface(registry: #create_name::Registry) -> #create_name::Registry {
                #implement
                let registry = registry.update_object(
                    <Self as #create_name::Object>::NAME,
                    <Self as #create_name::Object>::NAME,
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

fn impl_register_fns_trait(obj: &impl CommonInterfacable) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();
    let register_interface = impl_register_interface(obj).into_token_stream();
    Ok(quote! {
        #register_interface

        impl #create_name::RegisterFns for #object_ident {
            const REGISTER_FNS: &'static [fn (registry: #create_name::Registry) -> #create_name::Registry] = &[#object_ident::__register_interface];
        }
    })
}

impl ToTokens for ResolvedObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
        let impl_graphql_doc = common::impl_graphql_doc(self).into_token_stream();
        let impl_register_extras = impl_register_fns_trait(self).into_token_stream();

        tokens.extend(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_graphql_doc
            #impl_register_extras
        });
    }
}
