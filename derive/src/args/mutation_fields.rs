use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::args::ExpandObjectFields;
use crate::utils::common::CommonArg;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::impl_block::FromItemImpl;

pub struct MutationFields(ExpandObjectFields);

impl FromItemImpl for MutationFields {
    fn from_item_impl(item: &mut syn::ItemImpl) -> darling::Result<Self> {
        let fields = ExpandObjectFields::from_item_impl(item)?;
        Ok(Self(fields))
    }
}

fn validate_self(mutation: &MutationFields) -> darling::Result<TokenStream> {
    let errors: Vec<_> = mutation
        .0
        .methods
        .iter()
        .flat_map(|method| {
            let self_arg = method
                .args
                .iter()
                .find(|arg| matches!(arg.get_arg(), BaseFnArg::Receiver(_)));

            match self_arg {
                None => None,
                Some(self_arg) => {
                    let err =
                        darling::Error::custom("Mutation methods must not have a self argument")
                            .with_span(self_arg.get_arg())
                            .write_errors();
                    Some(quote! {
                        #err
                    })
                }
            }
        })
        .collect();
    Ok(quote! {
        #(#errors)*
    })
}

impl ToTokens for MutationFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fields = &self.0;
        let validate_self = validate_self(self).unwrap();
        tokens.extend(quote! {
            #validate_self
            #fields
        })
    }
}
