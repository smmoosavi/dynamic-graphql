use crate::args::common;
use crate::utils::common::CommonObject;
use crate::utils::derive_types::BaseStruct;
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;
use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{Generics, Path};

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,
}

from_derive_input!(
    ResolvedObject,
    WithAttributes<WithDoc<ResolvedObjectAttrs>, BaseStruct<(), Generics>>,
);

impl CommonObject for ResolvedObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
}

impl ToTokens for ResolvedObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
        let impl_graphql_doc = common::impl_graphql_doc(self).into_token_stream();

        tokens.extend(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_graphql_doc
        });
    }
}
