use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Generics;

use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::type_utils::get_owned_type;
use crate::utils::with_attributes::WithAttributes;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct MutationAttrs {}

from_derive_input!(
    Mutation,
    WithAttributes<MutationAttrs, NewtypeStruct<TupleField, Generics>>,
);

fn impl_mutation(mutation: &Mutation) -> darling::Result<TokenStream> {
    let ident = &mutation.ident;
    let crate_name = get_crate_name();
    let object_ident = &mutation.ident;
    let target = get_owned_type(&mutation.data.ty);
    let name = mutation.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = mutation.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::ParentType for #object_ident #ty_generics #where_clause {
            type Type = #target;
        }
        impl #impl_generics #crate_name::ExpandObject for #object_ident #ty_generics #where_clause {
            const NAME: &'static str = #name;
        }
        impl #crate_name::Mutation for #ident {}
        impl #crate_name::RegisterFns for #object_ident {
            const REGISTER_FNS: &'static [fn (registry: #crate_name::Registry) -> #crate_name::Registry] = &[];
        }
    })
}

impl ToTokens for Mutation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let code = impl_mutation(self).into_token_stream();
        tokens.extend(quote! {
            #code
        })
    }
}
