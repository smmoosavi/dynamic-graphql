use crate::utils::common::GetGenerics;
use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::macros::*;
use crate::utils::type_utils::get_owned_type;
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

from_derive_input!(ExpandObject, NewtypeStruct<TupleField, Generics>);

impl GetGenerics for ExpandObject {
    fn get_generics(&self) -> &Generics {
        &self.generics
    }
}

fn impl_expand_object(object: &ExpandObject) -> TokenStream {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    let target = get_owned_type(&object.data.ty);
    let name = object.ident.to_string();

    // parse generic
    let (impl_generics, ty_generics, where_clause) = object.get_generics().split_for_impl();

    quote! {
        impl #impl_generics #create_name::ExpandObject for #object_ident #ty_generics #where_clause {
            const NAME: &'static str = #name;
            type Target = #target;
        }
    }
}

fn impl_parent(object: &ExpandObject) -> TokenStream {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    let (impl_generics, ty_generics, where_clause) = object.get_generics().split_for_impl();

    quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
                fn parent(&self) -> &'a <Self as #create_name::ExpandObject>::Target {
                    self.0
                }
        }
    }
}

fn impl_from(object: &ExpandObject) -> TokenStream {
    let object_ident = &object.ident;
    let (impl_generics, ty_generics, where_clause) = object.get_generics().split_for_impl();
    let inner_type = &object.data.ty;

    quote! {
        impl #impl_generics From<#inner_type> for #object_ident #ty_generics #where_clause {
            fn from(target: #inner_type) -> Self {
                Self(target)
            }
        }
    }
}

impl ToTokens for ExpandObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_expand_object = impl_expand_object(self);
        let impl_parent = impl_parent(self);
        let impl_from = impl_from(self);
        tokens.extend(quote! {
            #impl_expand_object
            #impl_from
            #impl_parent
        });
    }
}
