extern crate proc_macro;

use darling::{FromDeriveInput, ToTokens};
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
