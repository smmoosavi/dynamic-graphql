use darling::{FromAttributes, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Generics, Path};

use crate::args::common;
use crate::args::common::{field_deprecation_code, get_enum_item_name, get_type_name};
use crate::utils::common::{CommonField, CommonObject, GetFields};
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::derive_types::{BaseEnum, UnitVariant};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::remove_attr::RemoteAttr;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct EnumVariantAttributes {
    #[darling(default)]
    name: Option<String>,

    #[darling(default)]
    deprecation: Deprecation,
}

#[derive(Default, Debug, Clone)]
pub struct EnumVariantContext {
    pub rename_items: Option<RenameRule>,
}

from_variant!(
    EnumVariant,
    WithAttributes<WithDoc<EnumVariantAttributes>, WithContext<EnumVariantContext, UnitVariant>>,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct EnumAttributes {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default)]
    pub rename_items: Option<RenameRule>,

    #[darling(default)]
    pub remote: Option<RemoteAttr>,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    Enum,
    WithAttributes<WithDoc<EnumAttributes>, BaseEnum<EnumVariant, Generics>>,
    ctx,
);

impl MakeContext<EnumVariantContext> for Enum {
    fn make_context(&self) -> EnumVariantContext {
        EnumVariantContext {
            rename_items: self.attrs.rename_items,
        }
    }
}

impl CommonObject for Enum {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn should_impl_type_name(&self) -> bool {
        !self.attrs.type_name
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

    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_items.as_ref()
    }
}

impl CommonField for EnumVariant {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Err(darling::Error::custom("Enum variant has no type").with_span(&self.ident))
    }

    fn get_skip(&self) -> bool {
        false
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }

    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        Ok(self.attrs.deprecation.clone())
    }

    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_items.as_ref()
    }
}

impl GetFields<EnumVariant> for Enum {
    fn get_fields(&self) -> darling::Result<&Vec<EnumVariant>> {
        Ok(&self.data)
    }
}

fn impl_enum(enm: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let enum_ident = enm.get_ident();
    let name = get_type_name(enm)?;

    let type_name = enm.should_impl_type_name().then_some(quote! {
         impl #crate_name::TypeName for #enum_ident {
            fn get_type_name() -> String {
                #name.into()
            }
        }
    });

    Ok(quote! {
        #type_name
        impl #crate_name::InputTypeName for #enum_ident {}
        impl #crate_name::OutputTypeName for #enum_ident {}
        impl #crate_name::Enum for #enum_ident {}
    })
}

fn impl_into_value_match_item(
    enm: &impl CommonObject,
    variant: &impl CommonField,
) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ty = enm.get_ident();
    let variant_ident = variant.get_ident()?;
    let variant_name = get_enum_item_name(variant)?;

    Ok(quote! {
        #ty::#variant_ident => {
            #crate_name::Value::Enum(
                #crate_name::Name::new(#variant_name)
            )
        }
    })
}

fn impl_into_value_match_items<T, F>(enm: &T) -> darling::Result<TokenStream>
where
    T: GetFields<F> + CommonObject,
    F: CommonField,
{
    Ok(enm
        .get_fields()?
        .iter()
        .map(|variant| impl_into_value_match_item(enm, variant).into_token_stream())
        .collect())
}

fn impl_into_value<T, F>(enm: &T) -> darling::Result<TokenStream>
where
    T: GetFields<F> + CommonObject,
    F: CommonField,
{
    let crate_name = get_crate_name();
    let enum_ident = enm.get_ident();
    let match_items = impl_into_value_match_items(enm)?;

    Ok(quote! {
        impl From<&#enum_ident> for #crate_name::Value {
            fn from(value: &#enum_ident) -> Self {
                match value {
                    #match_items
                }
            }
        }
    })
}

fn get_from_value_match_item(
    enm: &impl CommonObject,
    variant: &impl CommonField,
) -> darling::Result<TokenStream> {
    let ty = enm.get_ident();
    let variant_ident = variant.get_ident()?;
    let variant_name = get_enum_item_name(variant)?;

    Ok(quote! {
        #variant_name => {
            Ok(#ty::#variant_ident)
        }
    })
}

fn get_from_value_match_items<T, F>(enm: &T) -> darling::Result<TokenStream>
where
    T: GetFields<F> + CommonObject,
    F: CommonField,
{
    Ok(enm
        .get_fields()?
        .iter()
        .map(|variant| get_from_value_match_item(enm, variant).into_token_stream())
        .collect())
}

fn impl_from_value(enm: &Enum) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let enum_ident = enm.get_ident();
    let match_items = get_from_value_match_items(enm)?;

    Ok(quote! {
        impl #crate_name::FromValue for #enum_ident {
            fn from_value(__value: #crate_name::Result<#crate_name::dynamic::ValueAccessor>) -> #crate_name::Result<Self> {
                let __value = __value?;
                let string_value = __value.enum_name()?;
                match string_value {
                    #match_items
                    _ => Err(#crate_name::Error::new(
                        format!("Unknown variant `{}` for enum `{}`", string_value, <#enum_ident as #crate_name::Enum>::get_enum_type_name()),
                    )),
                }
            }
        }
    })
}

fn impl_into_remote_item(
    enum_ident: &syn::Ident,
    remote_path: &syn::Path,
    item: &EnumVariant,
) -> darling::Result<TokenStream> {
    let item_ident = item.get_ident()?;
    Ok(quote! {
            #enum_ident::#item_ident => #remote_path::#item_ident,
    })
}

fn impl_into_remote(enm: &Enum, remote_path: &syn::Path) -> darling::Result<TokenStream> {
    let enum_ident = enm.get_ident();
    let matches: TokenStream = enm
        .data
        .iter()
        .map(|item| impl_into_remote_item(enum_ident, remote_path, item).into_token_stream())
        .collect();
    Ok(quote! {
        impl From<#enum_ident> for #remote_path {
            fn from(value: #enum_ident) -> Self {
                match value {
                    #matches
                }
            }
        }
    })
}

fn impl_from_remote_item(
    enum_ident: &syn::Ident,
    remote_path: &syn::Path,
    item: &EnumVariant,
) -> darling::Result<TokenStream> {
    let item_ident = item.get_ident()?;
    Ok(quote! {
            #remote_path::#item_ident => #enum_ident::#item_ident,
    })
}

fn impl_from_remote(enm: &Enum, remote_path: &syn::Path) -> darling::Result<TokenStream> {
    let enum_ident = enm.get_ident();
    let matches: TokenStream = enm
        .data
        .iter()
        .map(|item| impl_from_remote_item(enum_ident, remote_path, item).into_token_stream())
        .collect();
    Ok(quote! {
        impl From<#remote_path> for #enum_ident {
            fn from(value: #remote_path) -> Self {
                match value {
                    #matches
                }
            }
        }
    })
}

fn impl_remote(enm: &Enum) -> darling::Result<TokenStream> {
    let Some(remote) = &enm.attrs.remote else {
        return Ok(quote! {});
    };
    let remote_path = &remote.0;

    let impl_into_remote = impl_into_remote(enm, remote_path)?;
    let impl_from_remote = impl_from_remote(enm, remote_path)?;

    Ok(quote! {
        #impl_into_remote
        #impl_from_remote
    })
}

fn register_item(variant: &impl CommonField) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let name = get_enum_item_name(variant)?;
    let description = common::field_description(variant)?;
    let deprecated = field_deprecation_code(variant)?;
    // todo rename field to item
    Ok(quote! {
        let field = #crate_name::dynamic::EnumItem::new(#name);
        #description
        #deprecated
        let object = object.item(field);
    })
}

fn register_items<T, F>(enm: &T) -> darling::Result<TokenStream>
where
    T: GetFields<F>,
    F: CommonField,
{
    Ok(enm
        .get_fields()?
        .iter()
        .map(|variant| register_item(variant).into_token_stream())
        .collect())
}

fn impl_register(enm: &Enum) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let enum_ident = enm.get_ident();
    let items = register_items(enm)?;
    let description = common::object_description(enm.get_doc()?.as_deref())?;
    let register_union = common::register_object_code();
    let register_attr = &enm.attrs.registers;
    // todo rename object to enm
    Ok(quote! {
        impl #crate_name::Register for #enum_ident {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {
                #( #register_attr )*
                let object = #crate_name::dynamic::Enum::new(<#enum_ident as #crate_name::Enum>::get_enum_type_name());
                #description
                #items
                #register_union
            }
        }
    })
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_enum = impl_enum(self).into_token_stream();
        let impl_into_value = impl_into_value(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned_by_value(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref_by_value(self).into_token_stream();
        let impl_from_value = impl_from_value(self).into_token_stream();
        let impl_remote = impl_remote(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_enum
            #impl_into_value
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_from_value
            #impl_remote
            #impl_register
        });
    }
}
