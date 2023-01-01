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

fn impl_resolve_owned(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::owned_any(self)))
            }
        }
    }
}

fn impl_resolve_ref(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveRef<'a> for #ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::borrowed_any(self)))
            }
        }
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = impl_object(self);
        let impl_resolve_owned = impl_resolve_owned(self);
        let impl_resolve_ref = impl_resolve_ref(self);

        tokens.extend(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
        });
    }
}
