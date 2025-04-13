use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Generics;
use syn::Path;

use crate::args::common;
use crate::utils::common::CommonField;
use crate::utils::common::CommonObject;
use crate::utils::common::EMPTY_ARGS;
use crate::utils::common::GetArgs;
use crate::utils::common::GetFields;
use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::BaseEnum;
use crate::utils::derive_types::NewtypeVariant;
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::MakeContext;
use crate::utils::with_context::WithContext;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct OneOfInputFieldAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct OneOfInputFieldContext {
    pub rename_fields: Option<RenameRule>,
}

from_variant!(
    OneOfInputField,
    WithAttributes<
        WithDoc<OneOfInputFieldAttrs>,
        WithContext<OneOfInputFieldContext, NewtypeVariant>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct OneOfInputAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    OneOfInput,
    WithAttributes<WithDoc<OneOfInputAttrs>, BaseEnum<OneOfInputField, Generics>>,
    ctx,
);

impl MakeContext<OneOfInputFieldContext> for OneOfInput {
    fn make_context(&self) -> OneOfInputFieldContext {
        OneOfInputFieldContext {
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonObject for OneOfInput {
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
}

impl CommonField for OneOfInputField {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&syn::Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Ok(&self.fields.ty)
    }

    fn get_skip(&self) -> bool {
        self.attrs.skip
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }

    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_fields.as_ref()
    }
}

impl GetFields<OneOfInputField> for OneOfInput {
    fn get_fields(&self) -> darling::Result<&Vec<OneOfInputField>> {
        Ok(&self.data)
    }
}

impl GetArgs<()> for OneOfInputField {
    fn get_args(&self) -> darling::Result<&Vec<()>> {
        Ok(&EMPTY_ARGS)
    }
}

fn get_define_field(field: &impl CommonField) -> darling::Result<TokenStream> {
    let description = common::field_description(field)?;
    let get_new_input_value_code = common::get_new_optional_input_value_code(field)?;
    Ok(quote! {
        #get_new_input_value_code
        #description
        let object = object.field(field);
    })
}

fn get_define_fields<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: CommonField,
{
    Ok(object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(|field| get_define_field(field).into_token_stream())
        .collect())
}

fn impl_register(object: &OneOfInput) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = &object.ident;
    let register_nested_types = common::get_nested_type_register_code(object).into_token_stream();

    let define_object = common::impl_define_input_object();
    let define_fields = get_define_fields(object)?;
    let description = common::object_description(object.get_doc()?.as_deref())?;
    let register_object_code = common::register_object_code();

    let register_attr = &object.attrs.registers;

    Ok(quote! {
        impl #crate_name::internal::Register for #ident {
            fn register(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry {

                #( #register_attr )*

                #register_nested_types

                #define_object
                let object = object.oneof();

                #description

                #define_fields

                #register_object_code
            }
        }
    })
}

fn get_item_ident(index: usize, ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("field{}", index), ident.span())
}

fn get_field_usage(index: usize, field: &impl CommonField) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let field_ident = field.get_ident()?;
    let field_name = common::get_input_field_name(field)?;
    let item = get_item_ident(index, field_ident);
    Ok(quote! {
        if let Some(__field) = __object.get(#field_name) {
            let #item = #crate_name::internal::FromValue::from_value(Ok(__field)).map_err(|e| e.into_field_error(#field_name))?;
            return Ok(Self::#field_ident(#item));
        }
    })
}

fn get_fields_usage<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: CommonField,
{
    let items: Vec<_> = object
        .get_fields()?
        .iter()
        .enumerate()
        .map(|(index, field)| get_field_usage(index, field).into_token_stream())
        .collect();

    Ok(quote! {
        #(#items)*
    })
}

fn no_field_error() -> TokenStream {
    let crate_name = get_crate_name();

    quote! {
        Err(#crate_name::internal::InputValueError::custom("Oneof input objects requires have exactly one field"))
    }
}

fn impl_from_value(object: &OneOfInput) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = object.get_ident();
    let fields_usage = get_fields_usage(object)?;
    let no_field_error = no_field_error();
    Ok(quote!(
        impl #crate_name::internal::FromValue for #ident {
            fn from_value(__value: #crate_name::Result<#crate_name::dynamic::ValueAccessor>) -> #crate_name::internal::InputValueResult<Self> {
                let __value = __value?;
                let __object = __value.object()?;
                #fields_usage
                #no_field_error
            }
        }
    ))
}

impl ToTokens for OneOfInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_input_object(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        let impl_from_value = impl_from_value(self).into_token_stream();
        tokens.extend(quote! {
            #impl_object
            #impl_register
            #impl_from_value
        });
    }
}
