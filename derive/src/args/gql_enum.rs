use crate::args::common;
use crate::args::common::{field_deprecation_code, get_enum_item_name, get_type_name};
use crate::utils::common::{CommonField, CommonObject};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::derive_types::{BaseEnum, UnitVariant};
use crate::utils::error::{GeneratorResult, IntoTokenStream};
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, SetContext, WithContext};
use crate::utils::with_doc::WithDoc;
use darling::util::SpannedValue;
use darling::{FromAttributes, FromDeriveInput, FromVariant, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Deref;
use syn::Variant;

#[derive(FromAttributes)]
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

pub struct EnumVariant(
    WithAttributes<WithDoc<EnumVariantAttributes>, WithContext<EnumVariantContext, UnitVariant>>,
);

impl Deref for EnumVariant {
    type Target = WithAttributes<
        WithDoc<EnumVariantAttributes>,
        WithContext<EnumVariantContext, UnitVariant>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SetContext for EnumVariant {
    type Context = <<Self as Deref>::Target as SetContext>::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.0.set_context(context);
    }
}

impl FromVariant for EnumVariant {
    fn from_variant(variant: &Variant) -> darling::Result<Self> {
        Ok(EnumVariant(FromVariant::from_variant(variant)?))
    }
}

#[derive(FromAttributes)]
#[darling(attributes(graphql))]
pub struct EnumAttributes {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_items: Option<RenameRule>,

    #[darling(default)]
    pub remote: Option<SpannedValue<String>>,
}

pub struct Enum(WithAttributes<WithDoc<EnumAttributes>, BaseEnum<EnumVariant>>);
impl Deref for Enum {
    type Target = WithAttributes<WithDoc<EnumAttributes>, BaseEnum<EnumVariant>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromDeriveInput for Enum {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let mut object = Self(FromDeriveInput::from_derive_input(input)?);
        object.0.set_context(object.make_context());
        Ok(object)
    }
}

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

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
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

    fn get_ident(&self) -> GeneratorResult<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> GeneratorResult<&syn::Type> {
        Err(darling::Error::custom("Enum variant has no type")
            .with_span(&self.ident)
            .into())
    }

    fn get_skip(&self) -> bool {
        false
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
        Ok(self.attrs.doc.clone())
    }

    fn get_deprecation(&self) -> GeneratorResult<Deprecation> {
        Ok(self.attrs.deprecation.clone())
    }

    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_items.as_ref()
    }
}

fn impl_enum(enm: &Enum) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let enum_ident = enm.get_ident();
    let name = get_type_name(enm)?;

    Ok(quote! {
         impl #create_name::GraphqlType for #enum_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::InputType for #enum_ident {}
        impl #create_name::OutputType for #enum_ident {}
        impl #create_name::Enum for #enum_ident {}
    })
}

fn impl_into_value_match_item(enm: &Enum, variant: &EnumVariant) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let ty = enm.get_ident();
    let variant_ident = variant.get_ident()?;
    let variant_name = get_enum_item_name(variant)?;

    Ok(quote! {
        #ty::#variant_ident => {
            #create_name::Value::Enum(
                #create_name::Name::new(#variant_name)
            )
        }
    })
}

fn impl_into_value_match_items(enm: &Enum) -> TokenStream {
    enm.data
        .iter()
        .map(|variant| impl_into_value_match_item(enm, variant).into_token_stream())
        .collect()
}

fn impl_into_value(enm: &Enum) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let enum_ident = enm.get_ident();
    let match_items = impl_into_value_match_items(enm);

    Ok(quote! {
        impl From<&#enum_ident> for #create_name::Value {
            fn from(value: &#enum_ident) -> Self {
                match value {
                    #match_items
                }
            }
        }
    })
}

fn get_from_value_match_item(enm: &Enum, variant: &EnumVariant) -> GeneratorResult<TokenStream> {
    let ty = enm.get_ident();
    let variant_ident = variant.get_ident()?;
    let variant_name = get_enum_item_name(variant)?;

    Ok(quote! {
        #variant_name => {
            Ok(#ty::#variant_ident)
        }
    })
}

fn get_from_value_match_items(enm: &Enum) -> TokenStream {
    enm.data
        .iter()
        .map(|variant| get_from_value_match_item(enm, variant).into_token_stream())
        .collect()
}

fn impl_from_value(enm: &Enum) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let enum_ident = enm.get_ident();
    let match_items = get_from_value_match_items(enm);

    Ok(quote! {
        impl #create_name::FromValue for #enum_ident {
            fn from_value(__value: #create_name::dynamic::ValueAccessor) -> #create_name::Result<Self> {
                let string_value = __value.enum_name()?;
                match string_value {
                    #match_items
                    _ => Err(#create_name::Error::new(
                        format!("Unknown variant `{}` for enum `{}`", string_value, <#enum_ident as #create_name::Enum>::NAME)
                    )),
                }
            }
        }
    })
}

fn impl_into_remote_item(
    enum_ident: &syn::Ident,
    remote_ident: &syn::Ident,
    item: &EnumVariant,
) -> GeneratorResult<TokenStream> {
    let item_ident = item.get_ident()?;
    Ok(quote! {
            #enum_ident::#item_ident => #remote_ident::#item_ident,
    })
}

fn impl_into_remote(enm: &Enum, remote_ident: &syn::Ident) -> GeneratorResult<TokenStream> {
    let enum_ident = enm.get_ident();
    let matches: TokenStream = enm
        .data
        .iter()
        .map(|item| impl_into_remote_item(enum_ident, remote_ident, item).into_token_stream())
        .collect();
    Ok(quote! {
        impl From<#enum_ident> for #remote_ident {
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
    remote_ident: &syn::Ident,
    item: &EnumVariant,
) -> GeneratorResult<TokenStream> {
    let item_ident = item.get_ident()?;
    Ok(quote! {
            #remote_ident::#item_ident => #enum_ident::#item_ident,
    })
}

fn impl_from_remote(enm: &Enum, remote_ident: &syn::Ident) -> GeneratorResult<TokenStream> {
    let enum_ident = enm.get_ident();
    let matches: TokenStream = enm
        .data
        .iter()
        .map(|item| impl_from_remote_item(enum_ident, remote_ident, item).into_token_stream())
        .collect();
    Ok(quote! {
        impl From<#remote_ident> for #enum_ident {
            fn from(value: #remote_ident) -> Self {
                match value {
                    #matches
                }
            }
        }
    })
}

fn impl_remote(enm: &Enum) -> GeneratorResult<TokenStream> {
    let Some(remote) = &enm.attrs.remote else {
        return Ok(quote! {});
    };
    let remote_ident = syn::Ident::new(remote, remote.span());

    let impl_into_remote = impl_into_remote(enm, &remote_ident)?;
    let impl_from_remote = impl_from_remote(enm, &remote_ident)?;

    Ok(quote! {
        #impl_into_remote
        #impl_from_remote
    })
}

fn register_item(variant: &EnumVariant) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let name = get_enum_item_name(variant)?;
    let description = common::field_description(variant)?;
    let deprecated = field_deprecation_code(variant)?;
    // todo rename field to item
    Ok(quote! {
        let field = #create_name::dynamic::EnumItem::new(#name);
        #description
        #deprecated
        let object = object.item(field);
    })
}

fn register_items(enm: &Enum) -> TokenStream {
    enm.data
        .iter()
        .map(|variant| register_item(variant).into_token_stream())
        .collect()
}

fn impl_register(enm: &Enum) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let enum_ident = enm.get_ident();
    let items = register_items(enm);
    let description = common::object_description(enm.get_doc()?.as_deref())?;
    // todo rename object to enm
    Ok(quote! {
        impl #create_name::Register for #enum_ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                let object = #create_name::dynamic::Enum::new(<#enum_ident as #create_name::GraphqlType>::NAME);
                #description
                #items
                registry.register_type(object)
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
