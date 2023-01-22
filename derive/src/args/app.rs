use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::{BaseStruct, TupleField};
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

pub type App = BaseStruct<TupleField, Generics>;

fn impl_create_schema(app: &App) -> TokenStream {
    let crate_name = get_crate_name();
    let ident = &app.ident;
    let (impl_generics, ty_generics, where_clause) = app.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn create_schema() -> #crate_name::dynamic::SchemaBuilder {
                let registry = #crate_name::Registry::new();
                let registry = registry.register::<Self>();
                registry.create_schema()
            }
        }
    }
}

fn impl_register(app: &App) -> TokenStream {
    let crate_name = get_crate_name();
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
    let (impl_generics, ty_generics, where_clause) = app.generics.split_for_impl();
    quote! {
        impl #impl_generics #crate_name::Register for #ident #ty_generics #where_clause {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {
                #registers
                registry
            }
        }
    }
}

impl ToTokens for App {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self);
        let impl_create_schema = impl_create_schema(self);
        tokens.extend(quote! {
            #impl_register
            #impl_create_schema
        });
    }
}
