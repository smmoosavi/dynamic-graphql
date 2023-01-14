use crate::args::common;
use crate::utils::common::{CommonField, CommonObject, GetFields};
use crate::utils::crate_name::get_create_name;
use crate::utils::derive_types::{BaseStruct, NamedField};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use darling::{FromAttributes, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct InputObjectFieldAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct InputObjectFieldContext {
    pub rename_fields: Option<RenameRule>,
}

from_field!(
    InputObjectField,
    WithAttributes<
        WithDoc<InputObjectFieldAttrs>,
        WithContext<InputObjectFieldContext, NamedField>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct InputObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,
}

from_derive_input!(
    InputObject,
    WithAttributes<WithDoc<InputObjectAttrs>, BaseStruct<InputObjectField>>,
    ctx,
);

impl MakeContext<InputObjectFieldContext> for InputObject {
    fn make_context(&self) -> InputObjectFieldContext {
        InputObjectFieldContext {
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonObject for InputObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_fields.as_ref()
    }
}

impl CommonField for InputObjectField {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&syn::Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Ok(&self.ty)
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

impl GetFields<InputObjectField> for InputObject {
    fn get_fields(&self) -> darling::Result<&Vec<InputObjectField>> {
        Ok(&self.data.fields)
    }
}

fn get_define_field(field: &impl CommonField) -> darling::Result<TokenStream> {
    let description = common::field_description(field)?;
    let get_new_input_value_code = common::get_new_input_value_code(field)?;
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

fn impl_register(object: &InputObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let ident = &object.ident;
    let define_object = common::impl_define_input_object();
    let define_fields = get_define_fields(object)?;
    let description = common::object_description(object.get_doc()?.as_deref())?;
    let register_object_code = common::register_object_code();
    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

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

fn get_field_value(index: usize, field: &InputObjectField) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let field_ident = field.get_ident()?;
    let item = get_item_ident(index, field_ident);
    let field_name = common::get_input_field_name(field)?;
    if field.get_skip() {
        return Ok(quote! {
            let #item = Default::default();
        });
    }
    Ok(quote! {
        let #item = #create_name::FromValue::from_value(__object.try_get(#field_name)?)?;
    })
}

fn get_fields_value(object: &InputObject) -> TokenStream {
    object
        .data
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| get_field_value(index, field).into_token_stream())
        .collect()
}

fn get_field_usage(index: usize, field: &impl CommonField) -> darling::Result<TokenStream> {
    let field_ident = field.get_ident()?;
    let item = get_item_ident(index, field_ident);
    Ok(quote! {
        #field_ident: #item,
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
        Ok(Self {
            #(#items)*
        })
    })
}

fn impl_from_value(object: &InputObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let ident = object.get_ident();
    let fields_value = get_fields_value(object);
    let fields_usage = get_fields_usage(object)?;
    Ok(quote!(
        impl #create_name::FromValue for #ident {
            fn from_value(__value: #create_name::dynamic::ValueAccessor) -> #create_name::Result<Self> {
                let __object = __value.object()?;
                #fields_value
                #fields_usage
            }
        }
    ))
}

impl ToTokens for InputObject {
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
