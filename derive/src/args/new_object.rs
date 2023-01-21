use darling::FromAttributes;
use syn::Generics;
use crate::args::mutation_root::MutationRootAttrs;
use crate::utils::derive_types::{BaseStruct, TupleField};
use crate::utils::macros::*;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct NewObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,
}


from_derive_input!(
    NewObject,
    WithAttributes<WithDoc<MutationRootAttrs>, BaseStruct<TupleField, Generics>>,
    ctx,
);
