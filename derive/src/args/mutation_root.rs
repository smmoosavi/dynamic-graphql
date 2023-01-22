use crate::args::simple_object::SimpleObjectAttrs;
use crate::args::SimpleObject;
use darling::ast::{Fields, Style};
use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Generics;

use crate::utils::derive_types::{BaseStruct, UnitStruct};
use crate::utils::macros::*;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct MutationRootAttrs {
    #[darling(default)]
    pub name: Option<String>,
}

from_derive_input!(
    MutationRoot,
    WithAttributes<WithDoc<MutationRootAttrs>, UnitStruct>,
    ctx,
);

impl From<MutationRoot> for SimpleObject {
    fn from(value: MutationRoot) -> Self {
        let name = value.0.attrs.inner.name;
        let doc = value.0.attrs.doc;
        let ident = value.0.inner.ident;

        SimpleObject(WithAttributes {
            attrs: WithDoc {
                doc,
                inner: SimpleObjectAttrs {
                    root: false,
                    mutation_root: true,
                    name,
                    rename_fields: None,
                    mark_as: vec![],
                    mark_with: vec![],
                    implement: vec![],
                },
            },
            inner: BaseStruct {
                ident,
                generics: Generics::default(),
                data: Fields::new(Style::Unit, vec![]),
            },
        })
    }
}

impl ToTokens for MutationRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let simple_object = SimpleObject::from(self.clone());
        let simple_object_code = simple_object.to_token_stream();
        tokens.extend(quote! {
            #simple_object_code
        })
    }
}
