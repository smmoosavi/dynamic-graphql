extern crate proc_macro;

use darling::FromDeriveInput;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod utils;

#[proc_macro_derive(Object, attributes(graphql))]
pub fn drive_command_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let object_args =
        match args::Object::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
            Ok(object_args) => object_args,
            Err(err) => return err.write_errors().into(),
        };
    match args::Object::generate(&object_args) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.write_errors().into(),
    }
}
