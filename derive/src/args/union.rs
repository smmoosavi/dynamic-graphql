use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Generics, Path, Type};

use crate::args::common;
use crate::utils::common::{CommonField, CommonObject, GetArgs, GetFields, EMPTY_ARGS};
use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::{BaseEnum, NewtypeVariant};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::type_utils::{get_owned_type, get_type_path};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

from_variant!(UnionItem, NewtypeVariant,);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct UnionAttrs {
    #[darling(default)]
    name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    Union,
    WithAttributes<WithDoc<UnionAttrs>, BaseEnum<UnionItem, Generics>>,
);

impl CommonObject for Union {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn should_impl_type_name(&self) -> bool {
        !self.attrs.type_name
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
        Ok(self.attrs.doc.clone())
    }
}

impl GetFields<UnionItem> for Union {
    fn get_fields(&self) -> darling::Result<&Vec<UnionItem>> {
        Ok(&self.data)
    }
}

impl GetArgs<()> for UnionItem {
    fn get_args(&self) -> darling::Result<&Vec<()>> {
        Ok(&EMPTY_ARGS)
    }
}

impl CommonField for UnionItem {
    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&Type> {
        Ok(&self.fields.ty)
    }

    fn get_skip(&self) -> bool {
        false
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(None)
    }
}

fn impl_union(union: &Union) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let name = common::get_type_name(union)?;
    let ident = union.get_ident();
    let type_name = union.should_impl_type_name().then_some(quote! {
        impl #crate_name::TypeName for #ident {
            fn get_type_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }
        }
    });
    Ok(quote! {
        #type_name
        impl #crate_name::OutputTypeName for #ident {}
        impl #crate_name::Union for #ident {}
    })
}

fn define_resolve_owned_match_pattern(
    union: &Union,
    item: &UnionItem,
) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let union_ident = union.get_ident();
    let variant_ident = &item.ident;
    let variant_type = get_type_path(&item.fields.ty)?;
    Ok(quote! {
        #union_ident::#variant_ident(value) => {
            #crate_name::Resolve::resolve(value,ctx).map(|value| value.map(|value| value.with_type(<#variant_type as #crate_name::Object>::get_object_type_name())))
        }
    })
}

fn define_resolve_owned_for_union(union: &Union) -> darling::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name();
    let ident = union.get_ident();

    let match_patterns = union
        .data
        .iter()
        .map(|item| define_resolve_owned_match_pattern(union, item).into_token_stream())
        .collect::<Vec<_>>();

    Ok(quote! {
        impl<'__dynamic_graphql_lifetime> #crate_name::ResolveOwned<'__dynamic_graphql_lifetime> for #ident {
            fn resolve_owned(self, ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<'__dynamic_graphql_lifetime>>> {
                match self {
                    #(#match_patterns),*
                }
            }
        }
    })
}

fn define_resolve_ref_match_pattern(
    union: &Union,
    item: &UnionItem,
) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let union_ident = union.get_ident();
    let variant_ident = &item.ident;
    let variant_type = get_type_path(&item.fields.ty)?;
    Ok(quote! {
        #union_ident::#variant_ident(value) => {
            #crate_name::Resolve::resolve(value,ctx).map(|value| value.map(|value| value.with_type(<#variant_type as #crate_name::Object>::get_object_type_name())))
        }
    })
}

fn define_resolve_ref_for_union(union: &Union) -> darling::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name();
    let ident = union.get_ident();

    let match_patterns = union
        .data
        .iter()
        .map(|item| define_resolve_ref_match_pattern(union, item).into_token_stream())
        .collect::<Vec<_>>();

    Ok(quote! {
        impl<'__dynamic_graphql_lifetime> #crate_name::ResolveRef<'__dynamic_graphql_lifetime> for #ident {
            fn resolve_ref(&'__dynamic_graphql_lifetime self, ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<'__dynamic_graphql_lifetime>>> {
                match self {
                    #(#match_patterns),*
                }
            }
        }
    })
}

fn define_union_code() -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    Ok(quote! {
        let object = #crate_name::dynamic::Union::new(<Self as #crate_name::Union>::get_union_type_name().as_ref());
    })
}

fn define_item(item: &UnionItem) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ty = get_owned_type(&item.fields.ty);
    Ok(quote! {
        let object = object.possible_type(<#ty as #crate_name::Object>::get_object_type_name().as_ref());
    })
}

fn define_items(union: &Union) -> darling::Result<TokenStream> {
    let items = union
        .data
        .iter()
        .map(|item| define_item(item).into_token_stream())
        .collect::<Vec<_>>();
    Ok(quote! {
        #(#items)*
    })
}

fn impl_register(union: &Union) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = union.get_ident();
    let register_nested_types = common::get_nested_type_register_code(union).into_token_stream();

    let define_union = define_union_code().into_token_stream();
    let description = union
        .get_doc()
        .and_then(|doc| common::object_description(doc.as_deref()))
        .into_token_stream();
    let define_items = define_items(union).into_token_stream();
    let register_union = common::register_object_code().into_token_stream();
    let register_attr = &union.attrs.registers;

    Ok(quote! {
        impl #crate_name::Register for #ident {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {

                #( #register_attr )*

                #register_nested_types

                #define_union

                #description

                #define_items

                #register_union
            }
        }
    })
}

impl ToTokens for Union {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_union = impl_union(self).into_token_stream();
        let resolve_owned = define_resolve_owned_for_union(self).into_token_stream();
        let resolve_ref = define_resolve_ref_for_union(self).into_token_stream();
        let register = impl_register(self).into_token_stream();

        tokens.extend(quote! {
            #impl_union
            #resolve_owned
            #resolve_ref
            #register
        });
    }
}
