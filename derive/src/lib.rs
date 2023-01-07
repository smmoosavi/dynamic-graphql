extern crate core;
extern crate proc_macro;

use crate::utils::impl_block::FromItemImpl;
use darling::{FromDeriveInput, ToTokens};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod utils;

#[proc_macro_derive(Object, attributes(graphql))]
pub fn drive_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::Object::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(ResolvedObject, attributes(graphql))]
pub fn drive_resolved_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::ResolvedObject::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn ResolvedObjectFields(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemImpl);
    let data = args::ResolvedObjectFields::from_item_impl(&mut item);
    let extension = match data {
        Ok(obj) => obj.into_token_stream(),
        Err(err) => err.write_errors(),
    };
    (quote! {
        #item
        #extension
    })
    .into()
}
