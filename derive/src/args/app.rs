use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::{BaseStruct, TupleField};
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

pub type App = BaseStruct<TupleField>;

fn impl_register(app: &App) -> TokenStream {
    let create_name = get_create_name();
    let ident = &app.ident;
    let registers: TokenStream = app
        .data
        .fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            quote! {
                let registry = registry.register::<#ty>();
            }
        })
        .collect();
    quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #registers
                registry
            }
        }
    }
}

impl ToTokens for App {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self);
        tokens.extend(impl_register);
    }
}
