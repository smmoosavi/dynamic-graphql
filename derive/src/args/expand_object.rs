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

    // parse generic
    let (impl_generics, ty_generics, where_clause) = object.get_generics().split_for_impl();

    quote! {
        impl #impl_generics #create_name::ExpandObject for #object_ident #ty_generics #where_clause {
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

impl ToTokens for ExpandObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_expand_object = impl_expand_object(self);
        let impl_parent = impl_parent(self);
        tokens.extend(quote! {
            #impl_expand_object
            #impl_parent
        });
    }
}
