use darling::{FromAttributes, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

use crate::utils::common::CommonObject;
use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::type_utils::get_owned_type;
use crate::utils::with_attributes::WithAttributes;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectAttrs {
    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(ExpandObject, WithAttributes<ExpandObjectAttrs, NewtypeStruct<TupleField, Generics>>);

impl CommonObject for ExpandObject {
    fn get_name(&self) -> Option<&str> {
        None
    }

    fn should_impl_type_name(&self) -> bool {
        false
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

fn impl_expand_object(object: &ExpandObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = &object.ident;
    let target = get_owned_type(&object.data.ty);
    let name = object.ident.to_string();

    // parse generic
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::ParentType for #object_ident #ty_generics #where_clause {
            type Type = #target;
        }
        impl #impl_generics #crate_name::ExpandObject for #object_ident #ty_generics #where_clause {
            fn get_expand_object_name() -> std::borrow::Cow<'static, str> {
                #name.into()
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

fn impl_registers_fn(object: &ExpandObject) -> darling::Result<TokenStream> {
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

fn impl_register_fns_trait(object: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();

    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();
    let turbofish_generics = ty_generics.as_turbofish();

    let q = quote! {
        impl #impl_generics #crate_name::RegisterFns for #object_ident #ty_generics #where_clause {
            const REGISTER_FNS: &'static [fn (registry: #crate_name::Registry) -> #crate_name::Registry] = &[
                #object_ident #turbofish_generics ::__registers,
            ];
        }
    };

    Ok(q)
}

impl ToTokens for ExpandObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_expand_object = impl_expand_object(self).into_token_stream();
        // let impl_parent = impl_parent(self).into_token_stream();
        let impl_from = impl_from(self).into_token_stream();
        let impl_register_fns_trait = impl_register_fns_trait(self).into_token_stream();
        let impl_registers_fn = impl_registers_fn(self).into_token_stream();
        tokens.extend(quote! {
            #impl_registers_fn
            #impl_expand_object
            #impl_from
            // #impl_parent
            #impl_register_fns_trait
        });
    }
}
