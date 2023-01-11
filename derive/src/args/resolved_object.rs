use crate::args::common;
use crate::utils::common::CommonObject;
use crate::utils::docs_utils::Doc;
use crate::utils::error::{GeneratorResult, IntoTokenStream};
use darling::FromAttributes;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

#[derive(FromDeriveInput)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct ResolvedObject {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub name: Option<String>,
}

impl CommonObject for ResolvedObject {
    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
        Ok(Doc::from_attributes(&self.attrs)?.doc)
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
