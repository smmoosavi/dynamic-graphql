use darling::FromAttributes;
use darling::ast::Fields;
use darling::ast::Style;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Generics;

use crate::args::SimpleObject;
use crate::args::simple_object::SimpleObjectAttrs;
use crate::utils::derive_types::BaseStruct;
use crate::utils::derive_types::UnitStruct;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct MutationRootAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    MutationRoot,
    WithAttributes<WithDoc<MutationRootAttrs>, UnitStruct>,
    ctx,
);

impl From<MutationRoot> for SimpleObject {
    fn from(value: MutationRoot) -> Self {
        let name = value.0.attrs.inner.name;
        let type_name = value.0.attrs.inner.type_name;
        let doc = value.0.attrs.doc;
        let ident = value.0.inner.ident;
        let registers = value.0.attrs.inner.registers;

        SimpleObject(WithAttributes {
            attrs: WithDoc {
                doc,
                inner: SimpleObjectAttrs {
                    root: false,
                    mutation_root: true,
                    name,
                    type_name,
                    rename_fields: None,
                    registers,
                    marks: vec![],
                    impls: vec![],
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
