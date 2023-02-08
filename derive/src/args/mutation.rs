use crate::utils::common::CommonObject;
use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Generics, Path};

use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::{NewtypeStruct, TupleField};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::type_utils::get_owned_type;
use crate::utils::with_attributes::WithAttributes;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct MutationAttrs {
    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    Mutation,
    WithAttributes<MutationAttrs, NewtypeStruct<TupleField, Generics>>,
);

impl CommonObject for Mutation {
    fn get_name(&self) -> Option<&str> {
        None
    }

    fn should_impl_type_name(&self) -> bool {
        false
    }

    fn get_ident(&self) -> &Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(None)
    }
}

fn impl_registers_fn(object: &Mutation) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = object.get_ident();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();

    let register_attr = &object.attrs.registers;

    Ok(quote! {
        impl #impl_generics #object_ident #ty_generics #where_clause {
            fn __registers(registry: #crate_name::Registry) -> #crate_name::Registry {
                #( #register_attr )*
                registry
            }
        }
    })
}

fn impl_mutation(mutation: &Mutation) -> darling::Result<TokenStream> {
    let ident = &mutation.ident;
    let crate_name = get_crate_name();
    let object_ident = &mutation.ident;
    let target = get_owned_type(&mutation.data.ty);
    let name = mutation.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = mutation.generics.split_for_impl();
    let turbofish_generics = ty_generics.as_turbofish();

    Ok(quote! {
        impl #impl_generics #crate_name::ParentType for #object_ident #ty_generics #where_clause {
            type Type = #target;
        }
        impl #impl_generics #crate_name::ExpandObject for #object_ident #ty_generics #where_clause {
            fn get_expand_object_name() -> String {
                #name.into()
            }
        }
        impl #crate_name::Mutation for #ident {}
        impl #crate_name::RegisterFns for #object_ident {
            const REGISTER_FNS: &'static [fn (registry: #crate_name::Registry) -> #crate_name::Registry] = &[
                #object_ident #turbofish_generics ::__registers,
            ];
        }
    })
}

impl ToTokens for Mutation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let code = impl_mutation(self).into_token_stream();
        let register_fn = impl_registers_fn(self).into_token_stream();
        tokens.extend(quote! {
            #register_fn
            #code
        })
    }
}
