use crate::args::common;
use crate::utils::common::CommonObject;
use crate::utils::derive_types::BaseStruct;
use crate::utils::error::IntoTokenStream;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, SetContext};
use crate::utils::with_doc::WithDoc;
use darling::FromAttributes;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::ops::Deref;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,
}

pub struct ResolvedObject(WithAttributes<WithDoc<ResolvedObjectAttrs>, BaseStruct<()>>);

impl Deref for ResolvedObject {
    type Target = WithAttributes<WithDoc<ResolvedObjectAttrs>, BaseStruct<()>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromDeriveInput for ResolvedObject {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let mut object = Self(FromDeriveInput::from_derive_input(input)?);
        object.0.set_context(object.make_context());
        Ok(object)
    }
}

impl CommonObject for ResolvedObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
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
