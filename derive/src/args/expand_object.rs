use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::type_utils::get_owned_type;
use crate::utils::with_context::{MakeContext, SetContext};
use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::Generics;

pub struct ExpandObject(NewtypeStruct<TupleField, Generics>);

impl Deref for ExpandObject {
    type Target = NewtypeStruct<TupleField, Generics>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromDeriveInput for ExpandObject {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let mut object = Self(FromDeriveInput::from_derive_input(input)?);
        object.0.set_context(object.make_context());
        Ok(object)
    }
}

fn impl_expand_object(object: &ExpandObject) -> TokenStream {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    let target = get_owned_type(&object.data.ty);
    quote! {
        impl #create_name::ExpandObject for #object_ident<'_> {
            type Target = #target;
        }
    }
}

fn impl_parent(object: &ExpandObject) -> TokenStream {
    let create_name = get_create_name();
    let object_ident = &object.ident;
    quote! {
        impl <'a> #object_ident<'a> {
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
