extern crate proc_macro;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod utils;

#[proc_macro_derive(Object, attributes(graphql))]
pub fn drive_command_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    args::Object::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|command_args| quote!(#command_args))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
