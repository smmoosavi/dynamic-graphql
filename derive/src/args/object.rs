use crate::utils::get_create_name;
use darling::ast::Data;
use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(FromField)]
#[darling(attributes(arg))]
pub struct ObjectField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}

#[derive(FromDeriveInput)]
#[darling(attributes(graphql))]
pub struct Object {
    pub ident: syn::Ident,
    pub data: Data<Ignored, ObjectField>,

    #[darling(default)]
    pub name: Option<String>,
}

fn impl_object(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let struct_name = ident.to_string();
    let name = object.name.as_ref().unwrap_or(&struct_name);
    let create_name = get_create_name();
    quote! {
        impl #create_name::Object for #ident {
            const NAME: &'static str = #name;
        }
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = impl_object(self);
        tokens.extend(quote! {
            #impl_object
        });
    }
}
