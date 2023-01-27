extern crate core;
extern crate proc_macro;

use darling::{FromDeriveInput, ToTokens};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::impl_block::{FromItemImpl, FromItemTrait};

mod args;
mod utils;

#[proc_macro_derive(SimpleObject, attributes(graphql))]
pub fn drive_simple_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::SimpleObject::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
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

#[proc_macro_derive(ExpandObject, attributes(graphql))]
pub fn drive_expand_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::ExpandObject::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
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

#[proc_macro_derive(InputObject, attributes(graphql))]
pub fn drive_input_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::InputObject::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Enum, attributes(graphql))]
pub fn drive_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::Enum::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(App, attributes(graphql))]
pub fn drive_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::App::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn ExpandObjectFields(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemImpl);
    let data = args::ExpandObjectFields::from_item_impl(&mut item);
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

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Interface(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemTrait);
    let data = args::Interface::from_item_trait(&mut item);
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

#[proc_macro_derive(Union, attributes(graphql))]
pub fn drive_union(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::Union::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(MutationRoot, attributes(graphql))]
pub fn drive_mutation_root(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::MutationRoot::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Mutation, attributes(graphql))]
pub fn drive_mutation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match args::Mutation::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(object_args) => object_args.into_token_stream().into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn MutationFields(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemImpl);
    let data = args::MutationFields::from_item_impl(&mut item);
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
